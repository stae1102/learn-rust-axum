use axum::{
    routing::get,
    response::{Json, Html},
    Router,
};

#[tokio::main]
async fn main() {
    let routes_hello: Router = Router::new()
        .route("/hello",
        get(|| async { Html("Hello <strong>World!!!</strong>") }),
    );

    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}
