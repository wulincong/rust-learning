use axum::{
    body::Body,
    routing::get,
    response::Json,
    Router,
};


use serde_json::{Value, json};
use axum::extract::{Path, Query, Json};
use std::collections::HashMap;



#[tokio::main(flavor="multi_thread", worker_threads = 4)]
async fn main() {

    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar));

    let app = Router::new().route("/", get(|| async{"hello world! "}));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();

}

async fn root() {}
async fn get_foo() {}
async fn post_foo() {}
async fn foo_bar() {}

// `Path` gives you the path parameters and deserializes them.
async fn path(Path(user_id): Path<u32>) {}

// `Query` gives you the query parameters and deserializes them.
async fn query(Query(params): Query<HashMap<String, String>>) {}

// Buffer the request body and deserialize it as JSON into a
// `serde_json::Value`. `Json` supports any type that implements
// `serde::Deserialize`.
async fn json(Json(payload): Json<serde_json::Value>) {}