use deadpool_lapin::{Manager, Pool};
use lapin::{ConnectionProperties, types::{LongString}};
use crate::rabbit::{rmq_listen};

mod rabbit;


#[tokio::main]
async fn main() -> Result<(), ()> {
    let addr =
        std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://rmq:rmq@127.0.0.1:5672/%2f".into());
    let manager = Manager::new(addr, ConnectionProperties::default().with_connection_name(LongString::from("conn")));
    let pool: Pool = deadpool::managed::Pool::builder(manager)
        .max_size(10)
        .build()
        .expect("can create pool");
    rmq_listen(pool.clone(), String::from("request")).await;
    Ok(())
}