
#[macro_use]
extern crate rocket;

mod endpoints;
mod repository;
mod schema;
mod models;
mod dtos;

use endpoints::{endpoints::{get_all_todos, insert_todo}, cors::Cors};


#[launch]
async fn rocket() -> _ {
    rocket::build().attach(Cors).mount(
        "/",
        routes![get_all_todos, insert_todo],
    )
}
