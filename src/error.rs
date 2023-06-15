use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use serde::Serialize;

// 여기서 Result 타입을 정의하며, 이는 결과와 Error를 Result 열거체로 타입 별칭
pub type Result<T> = core::result::Result<T, Error>;

// Enum을 복사시킬 수 있게 해준다.
// Rust에서는 직접적으로 상속을 지원하지 않고 트레이트를 통해 상속, 합성
#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
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

        // Create a placeholder Axum response.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the response.
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            
            // -- Auth.
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInRequestExt => {
                (StatusCode::UNAUTHORIZED, ClientError::NO_AUTH)
            }

            // -- Model.
            Self::TicketDeleteFailIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }

            // -- Fallback.
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            )
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}