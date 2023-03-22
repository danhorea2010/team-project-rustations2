use rocket::serde::json::Json;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::model::Todo;

pub struct Service {
    pool: Pool<Postgres>,
}


impl Service {
    pub async fn new(db_url: &str) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&db_url)
            .await
            .expect("Unable to connect to Postgres");

        return Service { pool };
    }

    pub async fn fetch_all(&self) -> Json<Vec<Todo>> {
        let todos: Vec<Todo> = sqlx::query_as!(Todo, "SELECT id, description, title FROM todo")
            .fetch_all(&self.pool)
            .await
            .expect("?");

        return Json(todos);
    }

    pub async fn insert(&self, title: &str, description: &str) {
        sqlx::query_as!(
            Todo, 
            "INSERT INTO todo(title, description) values($1, $2)", 
            title,description)
                .fetch_all(&self.pool)
                .await
                .expect("Unable to insert todo!");
    }

    pub async fn delete(&self, id: &i64) {
        sqlx::query_as!(
            Todo,
            "DELETE FROM todo where id=$1",
            id
        )
            .fetch_all(&self.pool)
            .await.expect("Could not delete todo item!");
    }

    pub async fn update(&self, id: i64, title: &str, description: &str) {
        sqlx::query_as!(
            Todo,
            "UPDATE todo set description = $1, title = $2 WHERE id = $3 ",
            description, title, id 
        )
            .fetch_all(&self.pool)
            .await.expect("Could not update todo item!");
    }
}