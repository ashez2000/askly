use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool, Row,
};
use uuid::Uuid;

use crate::{
    domain::{
        answer::Answer,
        question::Question,
        user::{Credential, User},
    },
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

    pub async fn add_question(&self, question: &Question, user_id: Uuid) -> Result<(), Error> {
        let sql = r"
            INSERT INTO questions (id, title, content, tags, user_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, title, content, tags
        ";

        match sqlx::query(sql)
            .bind(&question.id)
            .bind(&question.title)
            .bind(&question.content)
            .bind(&question.tags)
            .bind(&user_id)
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

    pub async fn is_question_owner(&self, question_id: Uuid, user_id: Uuid) -> Result<bool, Error> {
        match sqlx::query("SELECT * from questions where id = $1 and user_id = $2")
            .bind(question_id)
            .bind(user_id)
            .fetch_optional(&self.conn)
            .await
        {
            Ok(question) => Ok(question.is_some()),
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

    pub async fn add_answer(&self, input: Answer, user_id: Uuid) -> Result<Answer, Error> {
        let sql = r"
            INSERT INTO answers (id, content, question_id, user_id)
            VALUES ($1, $2, $3, $4)
            RETURNING id, content, question_id 
        ";

        match sqlx::query(sql)
            .bind(input.id)
            .bind(input.content)
            .bind(input.question_id)
            .bind(user_id)
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

    pub async fn is_answer_owner(&self, answer_id: Uuid, user_id: Uuid) -> Result<bool, Error> {
        match sqlx::query("SELECT * from answers where id = $1 and user_id = $2")
            .bind(answer_id)
            .bind(user_id)
            .fetch_optional(&self.conn)
            .await
        {
            Ok(answer) => Ok(answer.is_some()),
            Err(e) => Err(Error::DbError(e)),
        }
    }

    pub async fn add_user(&self, input: User) -> Result<(), Error> {
        let sql = r"
            INSERT INTO users (id, name, email, password)
            VALUES ($1, $2, $3, $4)
        ";

        match sqlx::query(sql)
            .bind(&input.id)
            .bind(&input.name)
            .bind(&input.email)
            .bind(&input.password)
            .execute(&self.conn)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::DbError(e)),
        }
    }

    pub async fn find_user_by_credential(&self, credential: Credential) -> Result<User, Error> {
        let sql = r"SELECT * FROM users WHERE email = $1";

        let user = sqlx::query(sql)
            .bind(credential.email)
            .map(|row: PgRow| User {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
                password: row.get("password"),
            })
            .fetch_one(&self.conn)
            .await
            .map_err(|_| Error::InvalidEmailPassword)?;

        match verify_password(&user.password, credential.password.as_bytes()) {
            Ok(verified) => {
                if verified {
                    Ok(user)
                } else {
                    Err(Error::InvalidEmailPassword)
                }
            }

            Err(_) => Err(Error::ServerError),
        }
    }
}

fn verify_password(hash: &str, password: &[u8]) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}
