use uuid::Uuid;
use warp::{reject::Rejection, reply::Reply};

use crate::{
    domain::question::{Question, QuestionInput},
    error::Error,
    store::Store,
};

pub async fn get_questions(store: Store) -> Result<impl Reply, Rejection> {
    let questions: Vec<Question> = store.questions.read().unwrap().values().cloned().collect();
    Ok(warp::reply::json(&questions))
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

pub async fn add_question(store: Store, input: QuestionInput) -> Result<impl Reply, Rejection> {
    let id = Uuid::new_v4().to_string();

    let question = Question {
        id: id.clone(),
        title: input.title,
        content: input.content,
        tags: input.tags,
    };

    store
        .questions
        .write()
        .unwrap()
        .insert(id, question.clone());

    Ok(warp::reply::json(&question))
}
