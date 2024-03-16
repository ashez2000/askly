use uuid::Uuid;
use warp::{reject::Rejection, reply::Reply};

use crate::{
    domain::answer::{Answer, NewAnswer},
    store::DbStore,
};

pub async fn get_answers(question_id: Uuid, store: DbStore) -> Result<impl Reply, Rejection> {
    match store.get_answers(question_id).await {
        Ok(answers) => Ok(warp::reply::json(&answers)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn add_answer(
    question_id: Uuid,
    store: DbStore,
    input: NewAnswer,
) -> Result<impl Reply, Rejection> {
    let answer = Answer {
        id: Uuid::new_v4(),
        content: input.content,
        question_id,
    };

    match store.add_answer(answer).await {
        Ok(answer) => Ok(warp::reply::json(&answer)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn delete_answer(id: Uuid, store: DbStore) -> Result<impl Reply, Rejection> {
    match store.delete_answer(id).await {
        Ok(_) => Ok(warp::reply::json(&true)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
