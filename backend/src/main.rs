use crate::service::Service;
use crate::endpoints::*;
use crate::cors::*;

#[macro_use]
extern crate rocket;

mod model;
mod service;
mod endpoints;
mod cors;

#[launch]
async fn rocket() -> _ {
    let db_url = dotenv::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
    let service = Service::new(&db_url).await;
    rocket::build().manage(service).attach(Cors).mount("/api/v1/todo", routes![fetch, insert,delete, update, all_options])
}
