use diesel::RunQueryDsl;

use crate::models::models::Todo;
use crate::repository::connection::database_connection::{establish_connection};
use crate::schema::todos::dsl::*;
use diesel::prelude::QueryResult;
use log::{error};

pub fn get_all() -> Vec<Todo>
{
    let connection = &mut establish_connection();
    let results = todos.load::<Todo>(connection);
    match results {
        QueryResult::Err(error) => {
            let result: Vec<Todo> = Vec::new();
            error!("{}", error);
            result
        },
        QueryResult::Ok(query_result) => query_result
    }
}

// pub fn insert(new_todo: NewTodo) -> todo
// {
//     let connection = &mut establish_connection();
//     let new_todo = vec![
//         (name.eq(new_todo.name), description.eq(new_todo.description), published.eq(false))
//     ];

//     diesel::insert_into(todo::table)
//         .values(&new_todo)
//         .get_result(conn)
//         .expect("Error saving new post")
// }

// pub fn update(update_todo: Todo) -> Todo
// {
//     let connection = &mut establish_connection();

//     let todo = diesel::update(posts.find(updateTodo.id))
//         .get_result::<Post>(connection)
//         .unwrap();
// }

// pub fn get(todoId: i32) -> vec<Todo>
// {
//     let connection = &mut establish_connection();
//     let selected = diesel::select(todos.filter(id.eq(todoId)))
//         .execute(connection)
//         .expect("Error deleting posts");
// }

// pub fn delete(todoId: i32) -> bool
// {
//     let connection = &mut establish_connection();
//     let num_deleted = diesel::delete(todos.filter(id.eq(todoId)))
//         .execute(connection)
//         .expect("Error deleting posts");
// }