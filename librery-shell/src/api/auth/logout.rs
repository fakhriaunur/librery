use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct LogoutResponse {
    pub message: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    responses(
        (status = OK, description = "Logout successful, cookie cleared", body = LogoutResponse),
        (status = INTERNAL_SERVER_ERROR, description = "Server error")
    ),
    tag = "auth"
)]
pub async fn logout() -> axum::response::Response {
    let clear_cookie = "auth_token=; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=0";

    let mut response = (
        StatusCode::OK,
        Json(LogoutResponse {
            message: "Logout successful".into(),
        }),
    )
        .into_response();

    response.headers_mut().insert(
        header::SET_COOKIE,
        clear_cookie.parse().unwrap(),
    );

    response
}
