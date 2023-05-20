use crate::models::models::{NewTodo, Todo};
use crate::repository::connection::database_connection::establish_connection;
use crate::schema::todos::dsl::*;
use diesel::prelude::QueryResult;
use diesel::prelude::*;
use log::error;

pub fn get_all() -> Vec<Todo> {
    let connection = &mut establish_connection();
    let results = todos.load::<Todo>(connection);
    match results {
        QueryResult::Err(error) => {
            let result: Vec<Todo> = Vec::new();
            error!("{}", error);
            result
        }
        QueryResult::Ok(query_result) => query_result,
    }
}

pub fn insert(new_todo: NewTodo) -> Vec<Todo> {
    let new_todos = vec![new_todo];
    let connection = &mut establish_connection();
    let results = diesel::insert_into(todos)
        .values(&new_todos)
        .get_results(connection);
    match results {
        QueryResult::Err(error) => {
            let result: Vec<Todo> = Vec::new();
            error!("{}", error);
            result
        }
        QueryResult::Ok(query_result) => query_result,
    }
}

pub fn update(update_todo: Todo) -> Todo {
    let connection = &mut establish_connection();

    let todo = diesel::update(todos.filter(id.eq(update_todo.id)))
        .set((
            title.eq(update_todo.title.clone()),
            visibility.eq(update_todo.visibility),
            description.eq(update_todo.description.clone()),
        ))
        .get_result(connection)
        .expect("Cannot update todo");

    return todo;
}

pub fn delete(todo_id: i32) -> bool {
    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(todos.filter(id.eq(todo_id)))
        .execute(connection)
        .expect("Error deleting todo");

    num_deleted != 0
}
