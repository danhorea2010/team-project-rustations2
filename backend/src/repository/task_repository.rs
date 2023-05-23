use crate::models::tasks::{NewTask, Task};
use crate::repository::connection::database_connection::establish_connection;
use crate::schema::tasks::dsl::*;
use diesel::prelude::QueryResult;
use diesel::prelude::*;
use log::error;

pub fn get(task_id: i32) -> Task {
    let connection = &mut establish_connection();
    let task = tasks.filter(id.eq(task_id))
            .get_result(connection)
            .expect("Cannot update todo");

    return task;
}

pub fn insert(new_task: NewTask) -> Vec<Task> {
    let new_tasks = vec![new_task];
    let connection = &mut establish_connection();
    let results = diesel::insert_into(tasks)
        .values(&new_tasks)
        .get_results(connection);
    match results {
        QueryResult::Err(error) => {
            let result: Vec<Task> = Vec::new();
            error!("{}", error);
            result
        }
        QueryResult::Ok(query_result) => query_result,
    }
}

pub fn update(update_task: Task) -> Task {
    let connection = &mut establish_connection();

    let task = diesel::update(tasks.filter(id.eq(update_task.id)))
        .set((
            task_started.eq(update_task.task_started),
            task_finished.eq(update_task.task_finished),
        ))
        .get_result(connection)
        .expect("Cannot update task");

    return task;
}
