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
async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODOL Implement real db/auth logic.
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // 쿠키 발급
    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}