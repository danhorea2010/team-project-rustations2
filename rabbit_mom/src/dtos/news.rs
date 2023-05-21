use serde::{Serialize, Deserialize};



// This is not stored in a database
// it is taken from 'https://www.cs.ubbcluj.ro'
#[derive(Serialize,Deserialize)]
pub struct NewsPost {
    pub title: String,
    pub description: String,
}

impl NewsPost {
    pub fn new(title:String, description: String) -> Self {
        NewsPost {
            title,
            description
        }
    }
}