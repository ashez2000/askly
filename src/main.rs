use warp::Filter;

mod domain;
mod store;

#[tokio::main]
async fn main() {
    let store = store::Store::new();
    let store = warp::any().map(move || store.clone());

    let hello = warp::get().map(|| format!("Hello, world!"));

    warp::serve(hello).run(([127, 0, 0, 1], 3000)).await;
}
