use crate::{Error, Result, web::AUTH_TOKEN};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use axum::routing::post;
use tower_cookies::{Cookies, Cookie};

/// 라우터 생성 함수
pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

/// 구조체의 제네릭에 구조체 넣는 패턴이 많은가?
/// 쿠키를 인자로 넣어서 바로 사용이 가능하다!
/// request body payload는 Json의 형식은 역시 구조체 struct로 정의.
/// 반환 타입도 Json의 Value 타입으로 반환
/// Value타입은 json! 매크로로 생성
async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODOL Implement real db/auth logic.
    // 향후에 logic 수정
    // 따로 함수로 빼두는 게 좋을 듯.
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // 쿠키 발급
    // Cookies는 쿠키 저장소에 접근할 수도 있어 보임
    // Cookie 구조체의 구현체 new 함수로 쿠키 생성
    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    // Create the success body.
    let body: Json<Value> = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
// 로그인 요청 페이로드에 대한 구조체
struct LoginPayload {
    username: String,
    pwd: String,
}