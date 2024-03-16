use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool, Row,
};
use uuid::Uuid;

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

    pub async fn get_question(&self, id: Uuid) -> Result<Question, Error> {
        match sqlx::query("SELECT * FROM questions WHERE id = $1")
            .bind(id)
            .map(|row: PgRow| Question {
                id: row.get("id"),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_one(&self.conn)
            .await
        {
            Ok(question) => Ok(question),
            Err(e) => Err(Error::DbError(e)),
        }
    }

    pub async fn add_question(&self, question: &Question) -> Result<(), Error> {
        let sql = r"
            INSERT INTO questions (id, title, content, tags)
            VALUES ($1, $2, $3, $4)
            RETURNING id, title, content, tags
        ";

        match sqlx::query(sql)
            .bind(&question.id)
            .bind(&question.title)
            .bind(&question.content)
            .bind(&question.tags)
            .execute(&self.conn)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::DbError(e)),
        }
    }
}
