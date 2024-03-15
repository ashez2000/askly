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

pub async fn get_question(id: String, store: Store) -> Result<impl Reply, Rejection> {
    match store.questions.read().unwrap().get(&id) {
        Some(question) => Ok(warp::reply::json(&question)),

        None => Err(warp::reject::custom(Error::NotFound(format!(
            "Question with id: {} not found",
            id
        )))),
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
    id: String,
    store: Store,
    input: NewQuestion,
) -> Result<impl Reply, Rejection> {
    let mut questions = store.questions.write().unwrap();

    if !questions.contains_key(&id) {
        return Err(warp::reject::custom(Error::NotFound(format!(
            "Question with id: {} not found",
            &id
        ))));
    };

    let question = Question {
        id: Uuid::new_v4(),
        title: input.title,
        content: input.content,
        tags: input.tags,
    };

    questions.insert(id, question.clone());

    Ok(warp::reply::json(&question))
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
