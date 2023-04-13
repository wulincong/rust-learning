use axum::{
    routing::get,
    Router,
};


#[tokio::main(flavor="multi_thread", worker_threads = 4)]
async fn main() {
    // println!("Hello, world!");
    let app = Router::new().route("/", get(|| async{"hello world! "}));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();

}
