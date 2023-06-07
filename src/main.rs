use std::net::SocketAddr;

use axum::{
    routing::get,
    response::{Html, IntoResponse},
    Router, extract::Query,
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello: Router = Router::new()
        .route("/hello",
        get(handler_hello),
    );

    // region:      --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8001));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

// region:    --- Handler Hello

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

// endregion: --- Handler Hello