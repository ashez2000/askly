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

    let routes = hello.or(get_questions);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
