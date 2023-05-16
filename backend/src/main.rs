use crate::cors::*;
use crate::endpoints::*;

#[macro_use]
extern crate rocket;

mod cors;
mod endpoints;
mod repository;
mod schema;
mod models;
mod dtos;

#[launch]
async fn rocket() -> _ {
    rocket::build().attach(Cors).mount(
        "/",
        routes![get_all_todos],
    )
}
