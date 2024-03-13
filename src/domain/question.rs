use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Question {
    pub id: String,
    pub title: String,
    pub contect: String,
    pub tags: Option<Vec<String>>,
}
