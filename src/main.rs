use std::net::SocketAddr;

use crate::model::ModelController;

pub use self::error::{Error, Result};

use axum::{
    routing::{get, get_service},
    response::{Html, IntoResponse, Response},
    Router, extract::{Query, Path}, middleware,
};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod ctx;
mod error;
mod model;
mod web;

// main 함수는 async를 붙일 수 없으나, tokio macro를 통해 사용 가능
#[tokio::main]
async fn main() -> Result<()> {
    // Initailize ModelController.
    let mc = ModelController::new().await?;

    // 해당 라우트에만 미들웨어 적용
    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));
    
    // 라우터를 생성
    let routes_all: Router = Router::new()
        .merge(routes_hello()) // 라우터 병합으로, 여러 개의 라우터를 하나의 라우터로 병합해서 사용
        .merge(web::routes_login::routes()) // 외부 크레이트에서 로그인 하는 라우트 병합
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper)) // middleware로 mapper를 두어서 응답 매핑
        .layer(CookieManagerLayer::new()) // 쿠키 매니저 사용
        .fallback_service(routes_static()); // 오류 발생 시 보여주는 정적 라우트

    // region:      --- Start Server
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8001)); // 127.0.0.1:8001에서 서버 구동할 수 있게 주소 열거체 생성
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service()) // 라우트를 사용
        .await
        .unwrap(); // Option, Result 내부 값을 가져옴

    Ok(())
}

// 응답에 대한 매퍼
// Response를 인자로 받아서, print 후 Response 반환.
// 그대로 소유권을 반환하기 때문에 인자로 소유권이 있는 Response 구조체를 인자로 사용
async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

// fallback에 대하여 오류를 처리함
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// region:    --- Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
// Query 인자의 구조체
// Optional한 name을 입력으로 받는다.
struct HelloParams { // query 사용을 위해 구조체 사용
    name: Option<String>,
}

// e.g., `/hello?name=Seongtae`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name: &str = params.name.as_deref().unwrap_or("World!"); // 소유권이 있는 변수값을 참조값으로 변환
    Html(format!("Hello <strong>{name}</strong>")) // name이 없으면 "World!" 출력
}

// e.g., `/hello2/Seongtae`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}

// endregion: --- Handler Hello