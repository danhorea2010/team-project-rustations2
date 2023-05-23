use diesel::{Insertable, Selectable, Queryable};
use crate::schema::tasks;


#[derive(Queryable, Selectable, Debug, Clone)]
pub struct Task {
    pub id: i32,
    pub task_started: bool,
    pub task_finished: bool,
}


#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub task_started: bool,
    pub task_finished: bool,
}





