use warp::{reject::Rejection, reply::Reply};

use crate::{domain::question::Question, store::Store};

pub async fn get_questions(store: Store) -> Result<impl Reply, Rejection> {
    let questions: Vec<Question> = store.questions.read().unwrap().values().cloned().collect();
    Ok(warp::reply::json(&questions))
}
