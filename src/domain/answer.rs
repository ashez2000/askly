use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct Answer {
    pub id: Uuid,
    pub content: String,
    pub question_id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewAnswer {
    pub content: String,
}
