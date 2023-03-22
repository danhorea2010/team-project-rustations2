
use rocket::serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Todo {
    pub id: i64,
    pub description: Option<String>,
    pub title: String,
}

#[derive(Debug, Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct IdDto {
    pub id: i64
}