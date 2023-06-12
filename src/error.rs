use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

// 여기서 Result 타입을 정의하며, 이는 결과와 Error를 Result 열거체로 타입 별칭
pub type Result<T> = core::result::Result<T, Error>;

// Enum을 복사시킬 수 있게 해준다.
// Rust에서는 직접적으로 상속을 지원하지 않고 트레이트를 통해 상속, 합성
#[derive(Debug, Clone)]
pub enum Error {
    LoginFail,

    // -- Auth errors.
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,

    // -- Model errors.
    // 구조체를 열거체에 넣을 수도 있다.
    TicketDeleteFailIdNotFound { id: u64 },
}

// Error에 대해서 구현체 적용
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR ").into_response()
    }
}