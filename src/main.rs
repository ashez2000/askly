use warp::Filter;

mod domain;
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

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store.clone())
        .and(warp::body::json())
        .and_then(routes::add_question);

    let routes = hello.or(get_questions).or(add_question);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
