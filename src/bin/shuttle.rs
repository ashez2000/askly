use warp::Reply;

#[shuttle_runtime::main]
async fn warp(
    #[shuttle_shared_db::Postgres] conn_str: String,
) -> shuttle_warp::ShuttleWarp<(impl Reply,)> {
    let store = askly::store::DbStore::new(&conn_str).await;
    sqlx::migrate!().run(&store.clone().conn).await.unwrap();

    let app = askly::build_routes(store).await;
    Ok(app.into())
}
