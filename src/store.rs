use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool, Row,
};

use crate::{domain::question::Question, error::Error};

#[derive(Clone)]
pub struct Store {
    pub questions: Arc<RwLock<HashMap<String, Question>>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            questions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DbStore {
    conn: PgPool,
}

impl DbStore {
    pub async fn new(url: &str) -> Self {
        let conn = PgPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await
            .expect("Db connection failed");

        Self { conn }
    }

    pub async fn get_questions(&self) -> Result<Vec<Question>, Error> {
        match sqlx::query("SELECT * FROM questions")
            .map(|row: PgRow| Question {
                id: row.get("id"),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.conn)
            .await
        {
            Ok(questions) => Ok(questions),
            Err(e) => Err(Error::DbError(e)),
        }
    }
}
