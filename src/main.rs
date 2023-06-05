use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/say-hello", get(say_hello))
        .route("/say-goodbye", get(say_goodbye));

    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
}

async fn root() -> String {
    return "Welcome!".to_string();
}

async fn say_hello() -> String {
    return "Hello!".to_string();
 }
 
 async fn say_goodbye() -> String {
    return "Goodbye!".to_string();
 }