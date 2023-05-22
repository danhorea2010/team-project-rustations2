use deadpool_lapin::{Manager, Pool, PoolError};
use futures::{join, StreamExt};
use lapin::{options::*, types::{FieldTable, LongString}, ConnectionProperties, BasicProperties};
use std::convert::Infallible;
use std::result::Result as StdResult;
use std::time::Duration;
use thiserror::Error as ThisError;
use tokio_amqp::*;
use warp::{Filter, Rejection, Reply};

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
            println!("received msg: {:?}", std::str::from_utf8(&delivery.data).unwrap());
            channel
                .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                .await?;
            send_message(get_pool(String::from("response_conn")).await, String::from("Hello there"),  String::from("response")).await;
            println!("I sent a message");
        }
    }
    Ok(())
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
