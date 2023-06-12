/*
    web 이하의 mod 파일을 통해서
    web과 관련된 module을 사용한다.
    export *
*/
// 권한 관련 미들웨어
pub mod mw_auth;
// 로그인 관련 라우트
pub mod routes_login;
// 티켓 관련 라우트
pub mod routes_tickets;

// 상수를 여기에 정의한다.
pub const AUTH_TOKEN: &str = "auth-token";