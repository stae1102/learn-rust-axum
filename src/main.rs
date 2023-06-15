// 주소값을 정수형으로 컨트롤
use std::net::SocketAddr;

// model 외부 파일 크레이트에서 ModelController라는 구조체를 불러옴
use crate::model::ModelController;

// 커스텀 error, 현재 file에서 Error, Result를 public으로 설정
// error module을 모두 가져와 main에 import하고 그 중에서 Error와 Result을 public으로 지정
pub use self::error::{Error, Result};

// axum 프레임워크 import
use axum::{
    // get은 Http method, get_service는 다른 service로 이동
    routing::{get, get_service},
    // 응답에 대한 의존성으로, Html을 렌더링. impl IntoResponse로 응답을 구현체로 적용
    response::{Html, IntoResponse, Response},
    // 요청을 받기 위해 Router를 사용하고, 요청에 대해 추적하기 위해서 Query, Path 임포트
    Router, extract::{Query, Path}, middleware, Json,
};
// 비식별화에 사용
use serde::Deserialize;
use serde_json::json;
// 쿠키 매니저
use tower_cookies::CookieManagerLayer;
// TODO: 뭔지 모르겠음.
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx;
mod error;
mod model;
mod web;

// main 함수는 async를 붙일 수 없으나, tokio macro를 통해 사용 가능
#[tokio::main]
async fn main() -> Result<()> {
    // Initailize ModelController.
    // ModelController의 new 메서드는 내부적으로 default 트레이트를 사용하기에
    // Mutex를 적용한 스레드를 사용할 수 있다.
    let mc = ModelController::new().await?;

    // 해당 라우트에만 미들웨어 적용
    /*
        routes_tickets 크레이터에 routes에는 ModelController를 clone하여 넣어준다.
        그 이유는 미들웨어가 해당 라우트의 상태(State)에 접근해야 하기 떄문이다.
     */
    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));
    
    // 라우터를 생성
    let routes_all: Router = Router::new()
        .merge(routes_hello()) // 라우터 병합으로, 여러 개의 라우터를 하나의 라우터로 병합해서 사용
        .merge(web::routes_login::routes()) // 외부 크레이트에서 로그인 하는 라우트 병합
        // api 경로 자식 경로는 모두 routes_apis에서 처리
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper)) // middleware로 mapper를 두어서 응답 매핑
        // ModelController에의 Context를 추출하며 유효성을 검사함.
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver, 
        ))
        .layer(CookieManagerLayer::new()) // 쿠키 매니저 사용
        // 서버로의 요청과 매치되지 않을 때 해당 서비스로 이동
        .fallback_service(routes_static()); // 오류 발생 시 보여주는 정적 라우트

    // region:      --- Start Server
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8000)); // 127.0.0.1:8001에서 서버 구동할 수 있게 주소 열거체 생성
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
    let uuid = Uuid::new_v4();

    // -- Get the eventual response error.
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // -- If client error, build the new response.
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            println!("   ->> client_error_body: {client_error_body}");

            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    println!("   ->> server log line - {uuid} - Error: {error_response:?}");

    println!();
    error_response.unwrap_or(res)
}

// fallback에 대하여 오류를 처리함
// / 경로에 중첩
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// region:    --- Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

// Deserialize - Stream을 객체로 변환
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