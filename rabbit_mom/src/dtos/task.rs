use crate::models::models::{Todo, NewTodo};
use serde::{Serialize, Deserialize};

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct TaskDTO {
    pub task_id: i32,
    pub path: String,
    pub parameter_id: Option<i32>,
    pub content: Option<String>,
}



