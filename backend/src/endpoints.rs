use crate::repository::todo_repository::{
    get_all
};
use rocket::{serde::json::Json};
use crate::dtos::todo::{TodoDTO};

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

// #[post("/", data = "<todo>")]
// pub async fn insert(todo: Json<Todo>, service: &State<Service>) -> Status {
//     Status::Created
// }

// #[delete("/", data = "<id>")]
// pub async fn delete(id: Json<IdDto>, service: &State<Service>) -> Status {
//     Status::Ok
// }

// #[put("/", data = "<todo>")]
// pub async fn update(todo: Json<Todo>, service: &State<Service>) -> Status {
//     let description = todo.description.clone().unwrap();
//     let title = &todo.title;
//     Status::Ok
// }
