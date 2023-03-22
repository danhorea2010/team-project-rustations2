use rocket::{State, serde::json::Json, http::Status};
use crate::{service::Service, model::Todo, model::IdDto};


#[get("/")]
pub async fn fetch(service : &State<Service>) -> Json<Vec<Todo>> {
    let todos = service.fetch_all().await;
    return todos;
}

#[post("/", data = "<todo>")]
pub async fn insert(todo: Json<Todo>, service: &State<Service>) -> Status {
    let description = todo.description.clone().unwrap();
    let title = &todo.title;

    service.insert(title, &description).await;    
    Status::Created
}

#[delete("/", data = "<id>")]
pub async fn delete(id: Json<IdDto>, service: &State<Service>) -> Status {
    service.delete(&id.id).await;
    Status::Ok
}

#[put("/", data = "<todo>")]
pub async fn update(todo: Json<Todo>, service: &State<Service>) -> Status {
    let description = todo.description.clone().unwrap();
    let title = &todo.title;

    service.update(todo.id, title, &description).await;
    Status::Ok
}
