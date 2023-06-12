use async_trait::async_trait;
use axum::RequestPartsExt;
use axum::extract::{FromRequestParts};
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::ctx::Ctx;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>
) -> Result<Response> {
    // Debug 매크로를 붙여서 바로 출력 가능
    println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");
    
    ctx?;
     
    Ok(next.run(req).await)
}

// region:    --- Ctx Extractor

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        // User the cookies extractor.
        let cookies = parts.extract::<Cookies>().await.unwrap();
        
        /* 
            Cookie extractor에서 우리가 원히는 키를 사용해 값을 찾아냄.
            찾아낸 후에 해당 쿠키의 값을 추출하고, map 콤비네이터로 값을 소유권 있는 String으로 변환
        */
        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        // Parse token.
        let (user_id, exp, sign) = auth_token
            .ok_or(Error::AuthFailNoAuthTokenCookie)
            .and_then(parse_token)?;

        // Token compoenets validation

        Ok(Ctx::new(user_id))
    }
}

// endreigon: --- Ctx Extractor

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
        &token
    )
    .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}