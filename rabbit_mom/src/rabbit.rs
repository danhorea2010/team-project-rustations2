use deadpool_lapin::{Manager, Pool, PoolError};
use futures::{join, StreamExt};
use lapin::{options::*, types::{FieldTable, LongString}, ConnectionProperties, BasicProperties};
use std::convert::Infallible;
use std::result::Result as StdResult;
use std::time::Duration;
use thiserror::Error as ThisError;
use tokio_amqp::*;
use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use crate::{
    dtos::{
        agenda::{AgendaDTO, NewAgendaDTO},
        news::NewsPost,
        task::TaskDTO,
        todo::{TodoDTO, NewTodoDTO}
    },
    repository::{
        agenda_repository::{delete_agenda_repo, get_all_agendas, insert_agenda_repo},
        todo_repository::{delete, get_all, insert, update},
        task_repository::{get as get_task, insert as insert_task, update as update_task},
    },
};

type WebResult<T> = StdResult<T, Rejection>;
type RMQResult<T> = StdResult<T, PoolError>;
type Result<T> = StdResult<T, Error>;
type Connection = deadpool::managed::Object<deadpool_lapin::Manager>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("rmq error: {0}")]
    RMQError(#[from] lapin::Error),
    #[error("rmq pool error: {0}")]
    RMQPoolError(#[from] PoolError),
}

impl warp::reject::Reject for Error {}

pub async fn get_pool(connection_name: String ) -> Pool
{
    let addr =
        std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://rmq:rmq@127.0.0.1:5672/%2f".into());
    let manager = Manager::new(addr, ConnectionProperties::default().with_connection_name(LongString::from(connection_name)));
    let pool: Pool = deadpool::managed::Pool::builder(manager)
        .max_size(10)
        .build()
        .expect("can create pool");
    pool
}


pub async fn rmq_listen(pool: Pool, queue_name: String) -> Result<()> {
    let mut retry_interval = tokio::time::interval(Duration::from_millis(100));
    loop {
        retry_interval.tick().await;
        println!("connecting rmq consumer...");
        match init_rmq_listen(pool.clone(), queue_name.clone()).await {
            Ok(any) => {
                println!("I got {:?}", any);
            }
            Err(e) => eprintln!("rmq listen had an error: {}", e),
        };
    }
}

async fn init_rmq_listen(pool: Pool, queue_name: String) -> Result<()> {
    let rmq_con = get_rmq_con(pool).await.map_err(|e| {
        eprintln!("could not get rmq con: {}", e);
        e
    })?;
    let channel = rmq_con.create_channel().await?;

    let queue = channel
        .queue_declare(
            queue_name.as_str(),
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;
    println!("Declared queue {:?}", queue);

    let mut consumer = channel
        .basic_consume(
            queue_name.as_str(),
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!("rmq consumer connected, waiting for messages");
    while let Some(delivery) = consumer.next().await {
        if let Ok((channel, delivery)) = delivery {
            let message_as_string = std::str::from_utf8(&delivery.data).unwrap();
            let task: TaskDTO = serde_json::from_str(message_as_string).unwrap();
            println!("received msg: {:?}", task);
            let mut task_from_db = get_task(task.task_id);
            println!("I got from db: {:?}", task_from_db);
            task_from_db.task_started = true;
            let mut updated_task = update_task(task_from_db);
            println!("I updated: {:?}", updated_task);
            updated_task.task_finished = true;
            let updated_task2 = update_task(updated_task);
            println!("I updated: {:?}", updated_task2);
            channel
                .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                .await?;
            let new_task_dto = TaskDTO {
                task_id: updated_task2.id,
                path: String::from("response"),
                parameter_id: None,
                content: Some(get_content(task.path, task.parameter_id, task.content).await)
            };
            send_message(get_pool(String::from("response_conn")).await, serde_json::to_string(&new_task_dto).unwrap(),  String::from("response")).await;
            println!("I sent a message");
        }
    }
    Ok(())
}
async fn get_content(path: String, parameter_id: Option<i32>, content: Option<String>) -> String 
{
    let result = match path.as_str() {
        "test" => String::from("test"),
        "get_all_todos" => {
            let todos = get_all();
            let result: Vec<TodoDTO> = todos.into_iter().map(|x| x.into()).collect();
            serde_json::to_string(&result).unwrap()
        },
        "get_all_agendas" => {
            let agendas = get_all_agendas();
            let result: Vec<AgendaDTO> = agendas.into_iter().map(|x| x.into()).collect();
            serde_json::to_string(&result).unwrap()
        },
        "add_agenda" => {
            let new_agenda: NewAgendaDTO = serde_json::from_str(content.unwrap().as_str()).unwrap();
            let agendas = insert_agenda_repo(new_agenda.into());
            let result: Vec<AgendaDTO> = agendas.into_iter().map(|x| x.into()).collect();
            serde_json::to_string(&result).unwrap()
        },
        "add_todo" => {
            let new_todo: NewTodoDTO = serde_json::from_str(content.unwrap().as_str()).unwrap();
            let todos = insert(new_todo.into());
            let result: Vec<TodoDTO> = todos.into_iter().map(|x| x.into()).collect();

            serde_json::to_string(&result).unwrap()
        },
        default => String::from("unkown")
    };
    result
}

async fn get_rmq_con(pool: Pool) -> RMQResult<Connection> {
    let connection = pool.get().await?;
    Ok(connection)
}

pub async fn send_message(pool: Pool, payload: String, queue_name: String) -> WebResult<impl Reply> {
    println!("I started");
    let rmq_con = get_rmq_con(pool).await.map_err(|e| {
        eprintln!("can't connect to rmq, {}", e);
        warp::reject::custom(Error::RMQPoolError(e))
    })?;
    println!("connection created");
    let channel = rmq_con.create_channel().await.map_err(|e| {
        eprintln!("can't create channel, {}", e);
        warp::reject::custom(Error::RMQError(e))
    })?;
    println!("channel created");
    let queue = channel
        .queue_declare(
            queue_name.as_str(),
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await;
    println!("Declared queue {:?}", queue);

    channel
        .basic_publish(
            "",
            queue_name.as_str(),
            BasicPublishOptions::default(),
            payload.as_str().as_bytes().to_vec(),
            BasicProperties::default(),
        )
        .await
        .map_err(|e| {
            eprintln!("can't publish: {}", e);
            warp::reject::custom(Error::RMQError(e))
        })?
        .await
        .map_err(|e| {
            eprintln!("can't publish: {}", e);
            warp::reject::custom(Error::RMQError(e))
        })?;
    Ok("OK")
}


// pub async fn send_message(pool: Pool, payload: String, queue_name: String) -> Result<()> {

//     let rmq_con = get_rmq_con(pool).await.map_err(|e| {
//         eprintln!("could not get rmq con: {}", e);
//         e
//     })?;
//     let channel = rmq_con.create_channel().await?;

//     let queue = channel
//         .queue_declare(
//             queue_name.as_str(),
//             QueueDeclareOptions::default(),
//             FieldTable::default(),
//         )
//         .await;
//     println!("Declared queue {:?}", queue);

//     channel
//         .basic_publish(
//             "",
//             queue_name.as_str(),
//             BasicPublishOptions::default(),
//             payload.as_str().as_bytes().to_vec(),
//             BasicProperties::default(),
//         )
//         .await
//         .map_err(|e| {
//             eprintln!("can't publish: {}", e);
//             warp::reject::custom(MyError::RMQError(e))
//         })?
//         .await
//         .map_err(|e| {
//             eprintln!("can't publish: {}", e);
//             warp::reject::custom(MyError::RMQError(e))
//         })?;
//     Ok(())
// }
