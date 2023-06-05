use axum::{
    Router,
    routing::get,
    extract::{Path, Query, Json},
};

use std::collections::HashMap;

// 'Path' gives you the path parameters and deserializes them.
async fn path(Path(user_id): Path<u32>) {}

// 'Query' gives you the query parameters and deserializes them.
async fn query(Query(params): Query<HashMap<String, String>>) {}

// Buffer the request body and deserialize it as JSON into a
// 'serde_json::Value'. 'Json' supports any type that implements
// 'serde::Deserilaize'.
async fn json(Json(payload): Json<serde_json::Value>) {}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar));

    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// calls one of these handlers
async fn root() -> String {
    "Hello World!".to_string()
}
async fn get_foo() {}
async fn post_foo() {}
async fn foo_bar() {}
