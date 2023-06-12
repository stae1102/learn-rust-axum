use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookies, Cookie};

use crate::ctx::Ctx;
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

/// 권한이 있는지 체크
/// 여기서 context에 접근할 수 있다.
/// ctx? 물음표 연산자로 값이 없으면 Error를 일으키도록 수정
pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>
) -> Result<Response> {
    // Debug 매크로를 붙여서 바로 출력 가능
    println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");
    
    ctx?;
     
    // 미들웨어이므로, next를 사용해 request로 넘겨줘야 함
    Ok(next.run(req).await)
}

/// ctx 검사
pub async fn mw_ctx_resolver<B> (
    // state를 통해 ModelController 에 접근한다.
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    // Cookies에서 쿠키를 찾아서 값을 찾아 소유권을 가지도록 매핑
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // Compute Result<Ctx>,
    let result_ctx = match auth_token
        // Option의 Some, None 체크
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        // 그 값에 대하여 함수 수행시킴, 여기서의 함수는 parse_token!
        .and_then(parse_token)
        {
            Ok((user_id, _exp, _sign)) => {
                // TODO: Token components validations.
                Ok(Ctx::new(user_id))
            },
            Err(e) => Err(e),
        };

    // 에러 발생 시 쿠키 삭제
    // matchs 매크로로 Error 열거체와 일치하는지 체크
    if result_ctx.is_err()
        && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
            cookies.remove(Cookie::named(AUTH_TOKEN))
        }

    // Store the ctx_result in the request extension.
        req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

// region:    --- Ctx Extractor

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequestExt)?
            .clone()
    }
}

// endreigon: --- Ctx Extractor

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
        &token // string slice
    )
    // regex_captures의 반환은 Option<(&str, str, str)>
    // 가장 처음 모든 값에 대해서 string slice, 그 이후는 () 안에 있는 값들을 반환
    .ok_or(Error::AuthFailTokenWrongFormat)?;

    // 참조한 문자열에 대해서 int를 parse
    // 소유권이 있는 정수형
    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}