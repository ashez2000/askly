use uuid::Uuid;
use warp::{filters::BoxedFilter, Filter, Reply};

pub mod domain;
pub mod error;
pub mod routes;
pub mod store;

use store::DbStore;

pub async fn build_routes(store: DbStore) -> BoxedFilter<(impl Reply,)> {
    let db_store = warp::any().map(move || store.clone());

    let hello = warp::get()
        .and(warp::path::end())
        .map(|| format!("Hello, world!"));

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(db_store.clone())
        .and_then(routes::get_questions);

    let get_question = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<Uuid>())
        .and(warp::path::end())
        .and(db_store.clone())
        .and_then(routes::get_question);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(routes::protect())
        .and(db_store.clone())
        .and(warp::body::json())
        .and_then(routes::add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<Uuid>())
        .and(warp::path::end())
        .and(routes::protect())
        .and(db_store.clone())
        .and(warp::body::json())
        .and_then(routes::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<Uuid>())
        .and(warp::path::end())
        .and(routes::protect())
        .and(db_store.clone())
        .and_then(routes::delete_question);

    let get_answers = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<Uuid>())
        .and(warp::path("answers"))
        .and(db_store.clone())
        .and_then(routes::get_answers);

    let add_answer = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::param::<Uuid>())
        .and(warp::path("answers"))
        .and(routes::protect())
        .and(db_store.clone())
        .and(warp::body::json())
        .and_then(routes::add_answer);

    let delete_answer = warp::delete()
        .and(warp::path("answers"))
        .and(warp::path::param::<Uuid>())
        .and(warp::path::end())
        .and(routes::protect())
        .and(db_store.clone())
        .and_then(routes::delete_answer);

    let signup = warp::post()
        .and(warp::path("signup"))
        .and(warp::path::end())
        .and(db_store.clone())
        .and(warp::body::json())
        .and_then(routes::signup);

    let signin = warp::post()
        .and(warp::path("signin"))
        .and(warp::path::end())
        .and(db_store.clone())
        .and(warp::body::json())
        .and_then(routes::signin);

    hello
        .or(get_questions)
        .or(get_question)
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .or(get_answers)
        .or(add_answer)
        .or(delete_answer)
        .or(signup)
        .or(signin)
        .recover(error::handle_rejection)
        .with(warp::trace::request())
        .boxed()
}
