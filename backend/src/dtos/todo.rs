use crate::models::{Todo};
use serde::ser::{Serialize, Serializer, SerializeStruct};

pub struct TodoDTO {
    pub id: i32,
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

impl Serialize for TodoDTO {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
    {
        let mut state = serializer.serialize_struct("TodoDTO", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("body", &self.description)?;
        state.serialize_field("visibility", &self.visibility)?;
        state.end()
    }
}