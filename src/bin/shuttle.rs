use std::env;

use shuttle_runtime::SecretStore;
use warp::Reply;

#[shuttle_runtime::main]
async fn warp(
    #[shuttle_shared_db::Postgres] conn_str: String,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_warp::ShuttleWarp<(impl Reply,)> {
    env::set_var("JWT_SECRET", secrets.get("JWT_SECRET").unwrap());

    let store = askly::store::DbStore::new(&conn_str).await;
    sqlx::migrate!().run(&store.clone().conn).await.unwrap();

    let app = askly::build_routes(store).await;
    Ok(app.into())
}
