use sqlx::{Pool, Sqlite};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Book {
    pub id: Uuid,
    pub description: String,
}

impl Book {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            description: String::from("default"),
        }
    }

    pub async fn save(self, pool: &Pool<Sqlite>) {
        sqlx::query("INSERT INTO book (id, description) VALUES (?, ?)")
            .bind(self.id)
            .bind(self.description)
            .execute(pool)
            .await
            .expect("Not able to save a book");
    }
}

pub async fn find_all(pool: &Pool<Sqlite>) -> Vec<Book> {
    sqlx::query_as("SELECT * FROM book")
        .fetch_all(pool)
        .await
        .expect("Can't read book")
}
