use crate::dtos::todo::{NewTodoDTO, TodoDTO};
use crate::models::tasks::{NewTask, Task};
use crate::models::models::{Todo};
use crate::{
    dtos::{
        agenda::{AgendaDTO, NewAgendaDTO},
        news::NewsPost,
        task::TaskDTO,
    },
    repository::{
        agenda_repository::{delete_agenda_repo, get_all_agendas, insert_agenda_repo},
        todo_repository::{delete, get_all, insert, update},
        task_repository::{get, insert as insert_task, update as update_task},
    },
};
use rocket::serde::json::Json;
use select::predicate::Class;
use select::{
    document::Document,
    predicate::{Name, Predicate},
};
use crate::rabbit::{get_pool, rmq_listen, send_message};
use serde_json::{Result, Value};

#[get("/")]
pub async fn get_all_todos() -> Json<Vec<TodoDTO>> {
    let new_task: NewTask = NewTask {
        task_started: false,
        task_finished: false
    };
    let mut task = insert_task(new_task);
    println!("{:?}", task);
    let new_task_dto = TaskDTO {
        task_id: task.into_iter().nth(0).unwrap().id,
        path: String::from("get_all_todos"),
        parameter_id: None,
        content: None
    };
    let _result = send_message(get_pool(String::from("conn")).await, String::from("request"), serde_json::to_string(&new_task_dto).unwrap()).await;
    let rabbitResult = rmq_listen(get_pool(String::from("conn2")).await);
    let taskResponse: TaskDTO = serde_json::from_str(rabbitResult.await.unwrap().as_str()).unwrap();
    let result: Vec<TodoDTO> = serde_json::from_str(taskResponse.content.unwrap().as_str()).unwrap();

    rocket::serde::json::Json(result)
}

#[get("/")]
pub async fn get_all_agenda() -> Json<Vec<AgendaDTO>> {
    let new_task: NewTask = NewTask {
        task_started: false,
        task_finished: false
    };
    let mut task = insert_task(new_task);
    println!("{:?}", task);
    let new_task_dto = TaskDTO {
        task_id: task.into_iter().nth(0).unwrap().id,
        path: String::from("get_all_agendas"),
        parameter_id: None,
        content: None
    };
    let _result = send_message(get_pool(String::from("conn")).await, String::from("request"), serde_json::to_string(&new_task_dto).unwrap()).await;
    let rabbitResult = rmq_listen(get_pool(String::from("conn2")).await);
    let taskResponse: TaskDTO = serde_json::from_str(rabbitResult.await.unwrap().as_str()).unwrap();
    let result: Vec<AgendaDTO> = serde_json::from_str(taskResponse.content.unwrap().as_str()).unwrap();

    rocket::serde::json::Json(result)
}

#[post("/", format = "json", data = "<new_agenda>")]
pub async fn insert_agenda(new_agenda: Json<NewAgendaDTO>) -> Json<Vec<AgendaDTO>> {
    
    let new_task: NewTask = NewTask {
        task_started: false,
        task_finished: false
    };
    let mut task = insert_task(new_task);
    println!("{:?}", task);
    let new_agenda_struct = new_agenda.into_inner();
    let new_task_dto = TaskDTO {
        task_id: task.into_iter().nth(0).unwrap().id,
        path: String::from("add_agenda"),
        parameter_id: None,
        content: Some(serde_json::to_string(&new_agenda_struct).unwrap())
    };
    let _result = send_message(get_pool(String::from("conn")).await, String::from("request"), serde_json::to_string(&new_task_dto).unwrap()).await;
    let rabbitResult = rmq_listen(get_pool(String::from("conn2")).await);
    let taskResponse: TaskDTO = serde_json::from_str(rabbitResult.await.unwrap().as_str()).unwrap();
    let result: Vec<AgendaDTO> = serde_json::from_str(taskResponse.content.unwrap().as_str()).unwrap();
    rocket::serde::json::Json(result)
}

#[post("/", format = "json", data = "<new_todo>")]
pub async fn insert_todo(new_todo: Json<NewTodoDTO>) -> Json<Vec<TodoDTO>> {
    let new_task: NewTask = NewTask {
        task_started: false,
        task_finished: false
    };
    let mut task = insert_task(new_task);
    println!("{:?}", task);
    let new_todo_struct = new_todo.into_inner();
    let new_task_dto = TaskDTO {
        task_id: task.into_iter().nth(0).unwrap().id,
        path: String::from("add_todo"),
        parameter_id: None,
        content: Some(serde_json::to_string(&new_todo_struct).unwrap())
    };
    let _result = send_message(get_pool(String::from("conn")).await, String::from("request"), serde_json::to_string(&new_task_dto).unwrap()).await;
    let rabbitResult = rmq_listen(get_pool(String::from("conn2")).await);
    let taskResponse: TaskDTO = serde_json::from_str(rabbitResult.await.unwrap().as_str()).unwrap();
    let result: Vec<TodoDTO> = serde_json::from_str(taskResponse.content.unwrap().as_str()).unwrap();

    rocket::serde::json::Json(result)
}

#[delete("/<id>")]
pub async fn delete_agenda(id: i32) -> Json<bool> {
    let new_task: NewTask = NewTask {
        task_started: false,
        task_finished: false
    };
    let mut task = insert_task(new_task);
    println!("{:?}", task);
    let new_task_dto = TaskDTO {
        task_id: task.into_iter().nth(0).unwrap().id,
        path: String::from("delete_agenda"),
        parameter_id: Some(id),
        content: None
    };
    let _result = send_message(get_pool(String::from("conn")).await, String::from("request"), serde_json::to_string(&new_task_dto).unwrap()).await;
    let rabbitResult = rmq_listen(get_pool(String::from("conn2")).await);
    let taskResponse: TaskDTO = serde_json::from_str(rabbitResult.await.unwrap().as_str()).unwrap();
    let result: bool = serde_json::from_str(taskResponse.content.unwrap().as_str()).unwrap();

    rocket::serde::json::Json(result)
}

#[delete("/<id>")]
pub async fn delete_todo(id: i32) -> Json<bool> {
    let new_task: NewTask = NewTask {
        task_started: false,
        task_finished: false
    };
    let mut task = insert_task(new_task);
    println!("{:?}", task);
    let new_task_dto = TaskDTO {
        task_id: task.into_iter().nth(0).unwrap().id,
        path: String::from("delete_todo"),
        parameter_id: Some(id),
        content: None
    };
    let _result = send_message(get_pool(String::from("conn")).await, String::from("request"), serde_json::to_string(&new_task_dto).unwrap()).await;
    let rabbitResult = rmq_listen(get_pool(String::from("conn2")).await);
    let taskResponse: TaskDTO = serde_json::from_str(rabbitResult.await.unwrap().as_str()).unwrap();
    let result: bool = serde_json::from_str(taskResponse.content.unwrap().as_str()).unwrap();

    rocket::serde::json::Json(result)
}

#[put("/", data = "<update_todo>")]
pub async fn update_todo(update_todo: Json<TodoDTO>) -> Json<TodoDTO> {
    let new_task: NewTask = NewTask {
        task_started: false,
        task_finished: false
    };
    let mut task = insert_task(new_task);
    println!("{:?}", task);
    let new_todo_struct = update_todo.into_inner();
    let new_task_dto = TaskDTO {
        task_id: task.into_iter().nth(0).unwrap().id,
        path: String::from("add_todo"),
        parameter_id: None,
        content: Some(serde_json::to_string(&new_todo_struct).unwrap())
    };
    let _result = send_message(get_pool(String::from("conn")).await, String::from("request"), serde_json::to_string(&new_task_dto).unwrap()).await;
    let rabbitResult = rmq_listen(get_pool(String::from("conn2")).await);
    let taskResponse: TaskDTO = serde_json::from_str(rabbitResult.await.unwrap().as_str()).unwrap();
    let result: TodoDTO = serde_json::from_str(taskResponse.content.unwrap().as_str()).unwrap();

    rocket::serde::json::Json(result)
}

#[get("/test")]
pub async fn test() -> Json<String> {
    let new_task: NewTask = NewTask {
        task_started: false,
        task_finished: false
    };
    let mut task = insert_task(new_task);
    println!("{:?}", task);
    let new_task_dto = TaskDTO {
        task_id: task.into_iter().nth(0).unwrap().id,
        path: String::from("test"),
        parameter_id: None,
        content: None
    };
    let _result = send_message(get_pool(String::from("conn")).await, String::from("request"), serde_json::to_string(&new_task_dto).unwrap()).await;
    let rabbitResult = rmq_listen(get_pool(String::from("conn2")).await);
    let finalResult: TaskDTO = serde_json::from_str(rabbitResult.await.unwrap().as_str()).unwrap();
    rocket::serde::json::Json(finalResult.content.unwrap())
}

#[get("/scrape")]
pub async fn scrape() -> Json<Vec<NewsPost>> {
    
    let new_task: NewTask = NewTask {
        task_started: false,
        task_finished: false
    };
    let mut task = insert_task(new_task);
    println!("{:?}", task);
    let new_task_dto = TaskDTO {
        task_id: task.into_iter().nth(0).unwrap().id,
        path: String::from("scrape"),
        parameter_id: None,
        content: None
    };
    let _result = send_message(get_pool(String::from("conn")).await, String::from("request"), serde_json::to_string(&new_task_dto).unwrap()).await;
    let rabbitResult = rmq_listen(get_pool(String::from("conn2")).await);
    let taskResponse: TaskDTO = serde_json::from_str(rabbitResult.await.unwrap().as_str()).unwrap();
    let result: Vec<NewsPost> = serde_json::from_str(taskResponse.content.unwrap().as_str()).unwrap();
    rocket::serde::json::Json(result)
}
