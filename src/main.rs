use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;
use uuid::Uuid;
use warp::Filter;

mod domain;
mod error;
mod routes;
mod store;

#[tokio::main]
async fn main() {
    let log_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "askly=info,warp=error".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    // In-memory store
    // let store = store::Store::new();
    // let store = warp::any().map(move || store.clone());

    // Database store
    let db_url = "postgresql://postgres:password@localhost:5432/askly";
    let db_store = store::DbStore::new(db_url).await;
    let db_store = warp::any().map(move || db_store.clone());

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
        .and(db_store.clone())
        .and(warp::body::json())
        .and_then(routes::add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<Uuid>())
        .and(warp::path::end())
        .and(db_store.clone())
        .and(warp::body::json())
        .and_then(routes::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<Uuid>())
        .and(warp::path::end())
        .and(db_store.clone())
        .and_then(routes::delete_question);

    let routes = hello
        .or(get_questions)
        .or(get_question)
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .recover(error::handle_rejection)
        .with(warp::trace::request());

    info!("Listening on port:3000");
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
