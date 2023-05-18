use crate::models::models::{Todo, NewTodo};
use serde::{Serialize, Deserialize};

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct TodoDTO {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub visibility: i16
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct NewTodoDTO {
    pub title: String,
    pub description: Option<String>,
    pub visibility: i16
}

impl TodoDTO {
    pub fn new(todo: Todo) -> TodoDTO {
        TodoDTO {
            id: todo.id,
            title: todo.title,
            description: todo.description,
            visibility: todo.visibility
        }
    }
}

impl From<NewTodo> for NewTodoDTO {
    fn from(todo: NewTodo) -> Self {
        NewTodoDTO { 
            title: todo.title.clone(), 
            description: todo.description.clone(), 
            visibility: todo.visibility 
        }
    }
}
