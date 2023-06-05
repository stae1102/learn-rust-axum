use axum::{
    body::Body,
    routing::get,
    response::Json,
    Router,
};
use serde_json::{Value, json};
// use serde_json::{Value, json};

// &'static str becomes a 200 OK with content-type: text/plain; charset=utf-8.
async fn plain_text() -> &'static str {
    "foo"
}

// Json gives a content-type of 'application/json and works with any type
// that implements serde::Serialize
async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/plain_text", get(plain_text))
        .route("/json", get(json));

    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// calls one of these handlers
async fn root() -> String {
    "Hello World!".to_string()
}
