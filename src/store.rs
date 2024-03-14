use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::domain::question::Question;

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
}
