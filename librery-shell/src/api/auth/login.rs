use axum::{
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use biscuit_auth::Biscuit;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::AppState;

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, ToSchema)]
pub struct LoginResponse {
    pub message: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = OK, description = "Login successful, token set in httpOnly cookie", body = LoginResponse),
        (status = UNAUTHORIZED, description = "Invalid credentials"),
        (status = INTERNAL_SERVER_ERROR, description = "Server error")
    ),
    tag = "auth"
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> axum::response::Response {
    let actor_id = librery_core::ActorId(payload.email.clone());

    let credentials = match state.credentials_repo.find_by_actor_id(&actor_id).await {
        Ok(Some(creds)) => creds,
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(LoginResponse {
                    message: "Invalid credentials".into(),
                }),
            )
                .into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(LoginResponse {
                    message: "Server error".into(),
                }),
            )
                .into_response();
        }
    };

    let password_valid = match bcrypt::verify(&payload.password, &credentials.password_hash) {
        Ok(valid) => valid,
        Err(_) => false,
    };

    if !password_valid {
        return (
            StatusCode::UNAUTHORIZED,
            Json(LoginResponse {
                message: "Invalid credentials".into(),
            }),
        )
            .into_response();
    }

    let keypair = state.biscuit_keypair;
    let expires_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64 + 86400;

    let builder = Biscuit::builder()
        .code(format!(
            r#"
            patron("{email}");
            check if time($time), $time < {expires_at};
            "#,
            email = payload.email,
            expires_at = expires_at
        ))
        .unwrap();

    let biscuit = builder.build(&keypair).unwrap();
    let token = biscuit.to_base64().unwrap();

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let _ = state.credentials_repo.update_last_login(&actor_id, now).await;

    let _cookie = format!(
        "auth_token={}; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=86400",
        token
    );

    let mut response = (
        StatusCode::OK,
        Json(LoginResponse {
            message: "Login successful".into(),
        }),
    )
        .into_response();

    response.headers_mut().insert(
        header::SET_COOKIE,
        _cookie.parse().unwrap(),
    );

    response
}

pub fn build_cookie_header(token: &str) -> String {
    format!(
        "auth_token={}; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=86400",
        token
    )
}
