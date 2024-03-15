use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Question {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewQuestion {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}
