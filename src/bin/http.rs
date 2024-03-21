use std::env;

use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    if let Err(_) = env::var("JWT_SECRET") {
        panic!("JWT_SECRET is not set");
    }

    if let Err(_) = env::var("DATABASE_URL") {
        panic!("DATABASE_URL is not set");
    }

    if let Err(_) = env::var("PORT") {
        panic!("PORT is not set");
    }

    let log_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "askly=info,warp=error".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let db_url = env::var("DATABASE_URL").unwrap();
    let store = askly::store::DbStore::new(&db_url).await;

    let port = env::var("PORT").unwrap().parse::<u16>().unwrap_or(3000);

    info!("Listening on port:{}", port);

    let app = askly::build_routes(store).await;

    warp::serve(app).run(([0, 0, 0, 0], port)).await;
}
