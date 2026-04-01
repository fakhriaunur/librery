use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

use crate::middleware::auth::AuthenticatedPatron;
use crate::AppState;

#[derive(Serialize, ToSchema)]
pub struct PatronProfileResponse {
    pub name: String,
    pub email: Option<String>,
    pub status: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/patrons/me",
    responses(
        (status = OK, description = "Patron profile", body = PatronProfileResponse),
        (status = UNAUTHORIZED, description = "Not authenticated"),
        (status = NOT_FOUND, description = "Patron not found"),
        (status = INTERNAL_SERVER_ERROR, description = "Server error")
    ),
    tag = "patrons"
)]
pub async fn get_patron_me(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::Extension(auth): axum::Extension<AuthenticatedPatron>,
) -> axum::response::Response {
    let actor = match state.actor_repo.find_by_id(&auth.patron_id).await {
        Ok(Some(actor)) => actor,
        Ok(None) => {
            return StatusCode::NOT_FOUND.into_response();
        }
        Err(_) => {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    (
        StatusCode::OK,
        Json(PatronProfileResponse {
            name: actor.name,
            email: actor.email,
            status: actor.status,
        }),
    )
        .into_response()
}
