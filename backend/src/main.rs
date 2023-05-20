
#[macro_use]
extern crate rocket;

mod endpoints;
mod repository;
mod schema;
mod models;
mod dtos;

use endpoints::{endpoints::{get_all_todos, insert_todo, update_todo, delete_todo}, cors::{Cors, all_options}};


#[launch]
async fn rocket() -> _ {
    rocket::build().attach(Cors).mount(
        "/todo",
        routes![get_all_todos, insert_todo, update_todo, delete_todo],
    )
    .mount("/", routes![all_options])

    
}
