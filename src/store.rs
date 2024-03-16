use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool, Row,
};
use uuid::Uuid;

use crate::{
    domain::{answer::Answer, question::Question},
    error::Error,
};

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

    pub async fn update_quesiton(&self, question: Question) -> Result<Question, Error> {
        let sql = r"
            UPDATE questions SET title = $1, content = $2, tags = $3
            WHERE id = $4
            RETURNING id, title, content, tags
        ";

        match sqlx::query(sql)
            .bind(question.title)
            .bind(question.content)
            .bind(question.tags)
            .bind(question.id)
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

    pub async fn delete_question(&self, id: Uuid) -> Result<(), Error> {
        match sqlx::query("DELETE FROM questions WHERE id = $1")
            .bind(id)
            .execute(&self.conn)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::DbError(e)),
        }
    }

    pub async fn get_answers(&self, question_id: Uuid) -> Result<Vec<Answer>, Error> {
        let sql = r"
            SELECT * FROM answers
            WHERE question_id = $1
        ";

        match sqlx::query(sql)
            .bind(question_id)
            .map(|row: PgRow| Answer {
                id: row.get("id"),
                content: row.get("content"),
                question_id: row.get("question_id"),
            })
            .fetch_all(&self.conn)
            .await
        {
            Ok(answers) => Ok(answers),
            Err(e) => Err(Error::DbError(e)),
        }
    }

    pub async fn add_answer(&self, input: Answer) -> Result<Answer, Error> {
        let sql = r"
            INSERT INTO answers (id, content, question_id)
            VALUES ($1, $2, $3)
            RETURNING id, content, question_id 
        ";

        match sqlx::query(sql)
            .bind(input.id)
            .bind(input.content)
            .bind(input.question_id)
            .map(|row: PgRow| Answer {
                id: row.get("id"),
                content: row.get("content"),
                question_id: row.get("question_id"),
            })
            .fetch_one(&self.conn)
            .await
        {
            Ok(answer) => Ok(answer),
            Err(e) => Err(Error::DbError(e)),
        }
    }

    pub async fn delete_answer(&self, id: Uuid) -> Result<(), Error> {
        match sqlx::query("DELETE FROM answers WHERE id = $1")
            .bind(id)
            .execute(&self.conn)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::DbError(e)),
        }
    }
}
