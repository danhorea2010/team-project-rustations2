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
    let is_deleted = delete_agenda_repo(id);
    rocket::serde::json::Json(is_deleted)
}

#[delete("/<id>")]
pub async fn delete_todo(id: i32) -> Json<bool> {
    let is_deleted = delete(id);
    rocket::serde::json::Json(is_deleted)
}

#[put("/", data = "<todo>")]
pub async fn update_todo(todo: Json<TodoDTO>) -> Json<TodoDTO> {
    let updated_todo = update(todo.into_inner().into());
    let updated_todo_dto: TodoDTO = updated_todo.into();

    rocket::serde::json::Json(updated_todo_dto)
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
    let url = "https://www.cs.ubbcluj.ro";
    let response = reqwest::get(url).await.unwrap().text().await.unwrap();

    let document = Document::from(response.as_str());

    let mut results = Vec::new();

    for div in document.find(Class("post-wrap").and(Class("clearfix"))) {
        let title = div
            .find(Name("h2"))
            .next()
            .and_then(|h2| h2.find(Name("a")).next())
            .map(|a| a.text())
            .unwrap_or_default();

        let description = div
            .find(Class("entry").and(Class("clearfix")))
            .next()
            .map(|entry| entry.text())
            .unwrap_or_default();

        results.push(NewsPost::new(title, description));
    }

    rocket::serde::json::Json(results)
}
