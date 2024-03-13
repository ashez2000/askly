use warp::Filter;

mod domain;
mod error;
mod routes;
mod store;

#[tokio::main]
async fn main() {
    let store = store::Store::new();
    let store = warp::any().map(move || store.clone());

    let hello = warp::get()
        .and(warp::path::end())
        .map(|| format!("Hello, world!"));

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store.clone())
        .and_then(routes::get_questions);

    let get_question = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store.clone())
        .and_then(routes::get_question);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store.clone())
        .and(warp::body::json())
        .and_then(routes::add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store.clone())
        .and(warp::body::json())
        .and_then(routes::update_question);

    let routes = hello
        .or(get_questions)
        .or(get_question)
        .or(add_question)
        .or(update_question)
        .recover(error::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
