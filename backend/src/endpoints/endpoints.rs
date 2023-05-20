use crate::{repository::{todo_repository::{
    get_all,
    insert, update, delete
}, agenda_repository::{get_all_agendas, insert_agenda_repo, delete_agenda_repo, update_agenda_repo}}, dtos::{agenda::{AgendaDTO, NewAgendaDTO}, news::NewsPost}};
use rocket::{serde::json::Json};
use crate::dtos::todo::{TodoDTO, NewTodoDTO};
use select::{document::Document, predicate::{Predicate, Name}};
use select::predicate::Class;


#[get("/")]
pub async fn get_all_todos() -> Json<Vec<TodoDTO>> {
    let todos = get_all();
    let result: Vec<TodoDTO> = todos
        .into_iter()
        .map(|x| x.into()).collect();

    rocket::serde::json::Json(result)
}

#[get("/")]
pub async fn get_all_agenda() -> Json<Vec<AgendaDTO>> {
    let agendas = get_all_agendas();
    let result: Vec<AgendaDTO> = agendas
        .into_iter()
        .map(|x| x.into()).collect();

    rocket::serde::json::Json(result)
}

#[post("/", format="json", data = "<new_agenda>")]
pub async fn insert_agenda(new_agenda: Json<NewAgendaDTO>) -> Json<Vec<AgendaDTO>> {
    let agendas = insert_agenda_repo(new_agenda.into_inner().into());

    let result: Vec<AgendaDTO> =
        agendas
        .into_iter()
        .map(|x| x.into())
        .collect();

    rocket::serde::json::Json(result)
}

#[post("/", format = "json", data = "<new_todo>")]
pub async fn insert_todo(new_todo: Json<NewTodoDTO>) -> Json<Vec<TodoDTO>> {
    let todos = insert(new_todo.into_inner().into());
    let result: Vec<TodoDTO> = 
        todos
        .into_iter()
        .map(|x| x.into())
        .collect();

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

        results.push(NewsPost::new(title,description));

    }


    rocket::serde::json::Json(results)
}
