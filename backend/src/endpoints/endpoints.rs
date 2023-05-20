use crate::{repository::todo_repository::{
    get_all,
    insert, update, delete
}};
use rocket::{serde::json::Json};
use crate::dtos::todo::{TodoDTO, NewTodoDTO};

#[get("/")]
pub async fn get_all_todos() -> Json<Vec<TodoDTO>> {
    let todos = get_all();
    let mut result: Vec<TodoDTO> = Vec::new();
    for todo in todos.iter() {
        let todo_dto = TodoDTO::new(todo.clone());
        result.push(todo_dto);
    }
    rocket::serde::json::Json(result)
}

#[post("/", format = "json", data = "<new_todo>")]
pub async fn insert_todo(new_todo: Json<NewTodoDTO>) -> Json<Vec<TodoDTO>> {
    let todos = insert(new_todo.into_inner().into());
    let mut result: Vec<TodoDTO> = Vec::new();
    for todo in todos.iter() {
        let todo_dto = TodoDTO::new(todo.clone());
        result.push(todo_dto);
    }
    rocket::serde::json::Json(result)
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
