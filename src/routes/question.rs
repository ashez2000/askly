use uuid::Uuid;
use warp::{reject::Rejection, reply::Reply};

use crate::{
    domain::question::{NewQuestion, Question},
    error::Error,
    store::{DbStore, Store},
};

pub async fn get_questions(store: DbStore) -> Result<impl Reply, Rejection> {
    match store.get_questions().await {
        Ok(questions) => Ok(warp::reply::json(&questions)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn get_question(id: Uuid, store: DbStore) -> Result<impl Reply, Rejection> {
    match store.get_question(id).await {
        Ok(question) => Ok(warp::reply::json(&question)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn add_question(store: DbStore, input: NewQuestion) -> Result<impl Reply, Rejection> {
    let question = Question {
        id: Uuid::new_v4(),
        title: input.title,
        content: input.content,
        tags: input.tags,
    };

    match store.add_question(&question).await {
        Ok(_) => Ok(warp::reply::json(&question)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn update_question(
    id: Uuid,
    store: DbStore,
    input: NewQuestion,
) -> Result<impl Reply, Rejection> {
    let question = Question {
        id,
        title: input.title,
        content: input.content,
        tags: input.tags,
    };

    match store.update_quesiton(question).await {
        Ok(question) => Ok(warp::reply::json(&question)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn delete_question(id: String, store: Store) -> Result<impl Reply, Rejection> {
    match store.questions.write().unwrap().remove(&id) {
        Some(question) => Ok(warp::reply::json(&question)),

        None => Err(warp::reject::custom(Error::NotFound(format!(
            "Question with id: {} not found",
            &id,
        )))),
    }
}
