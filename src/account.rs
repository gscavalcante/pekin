use std::ptr::null;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

#[derive(Debug, sqlx::Type)]
pub enum Category {
    ASSET,
    EQUITY,
    INCOME,
    EXPENSE,
    LIABILITY,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Account {
    id: Uuid,
    name: String,
    description: Option<String>,
    category: Category,
}

impl Account {
    pub fn new(name: &str, description: Option<String>, category: Category) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from(name),
            description,
            category,
        }
    }

    pub async fn save(self, pool: &Pool<Sqlite>) {
        sqlx::query("INSERT INTO account (id, name, description, category) VALUES (?, ?, ?, ?)")
            .bind(self.id)
            .bind(self.name)
            .bind(self.description)
            .bind(self.category)
            .execute(pool)
            .await
            .expect("Not able to save an account");
    }
}

pub async fn find_all(pool: &Pool<Sqlite>) -> Vec<Account> {
    sqlx::query_as("SELECT * FROM account")
        .fetch_all(pool)
        .await
        .expect("Can't read account")
}
