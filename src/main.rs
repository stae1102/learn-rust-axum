use std::net::SocketAddr;

use axum::{
    routing::get,
    response::{Html},
    Router,
};

#[tokio::main]
async fn main() {
    let routes_hello: Router = Router::new()
        .route("/hello",
        get(|| async { Html("Hello <strong>World!!!</strong>") }),
    );

    // region:      --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}
