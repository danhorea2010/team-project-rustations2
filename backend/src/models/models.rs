// Generated by diesel_ext

use diesel::Queryable;
use diesel::Insertable;
use crate::schema::todos;

#[derive(Queryable, Debug, Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub visibility: i16,
}

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo {
    pub title: String,
    pub description: Option<String>,
    pub visibility: i16,
}