use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

use crate::middleware::auth::AuthenticatedPatron;
use crate::AppState;

#[derive(Serialize, ToSchema)]
pub struct LoanResponse {
    pub item_id: String,
    pub checked_out_at: i64,
}

#[derive(Serialize, ToSchema)]
pub struct LoansResponse {
    pub loans: Vec<LoanResponse>,
}

#[derive(Serialize, ToSchema)]
pub struct HoldResponse {
    pub item_id: String,
    pub placed_at: i64,
}

#[derive(Serialize, ToSchema)]
pub struct HoldsResponse {
    pub holds: Vec<HoldResponse>,
}

#[utoipa::path(
    get,
    path = "/api/v1/patrons/{id}/loans",
    responses(
        (status = OK, description = "List of active loans for the patron", body = LoansResponse),
        (status = UNAUTHORIZED, description = "Not authenticated"),
        (status = FORBIDDEN, description = "Not authorized to view this patron's loans"),
        (status = INTERNAL_SERVER_ERROR, description = "Server error")
    ),
    tag = "patrons"
)]
pub async fn get_patron_loans(
    State(state): State<AppState>,
    Path(patron_id): Path<String>,
    axum::Extension(auth): axum::Extension<AuthenticatedPatron>,
) -> axum::response::Response {
    if auth.patron_id != patron_id {
        return StatusCode::FORBIDDEN.into_response();
    }

    let events = match state.circulation_repo.get_events().await {
        Ok(events) => events,
        Err(_) => {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let mut active_loans: std::collections::HashMap<String, i64> = std::collections::HashMap::new();

    for event in events {
        match event {
            librery_core::CirculationEvent::ItemCheckedOut {
                item_id,
                patron_id: event_patron_id,
                timestamp,
                ..
            } if event_patron_id.0 == patron_id => {
                active_loans.insert(item_id.0.clone(), timestamp);
            }
            librery_core::CirculationEvent::ItemRenewed {
                item_id,
                patron_id: event_patron_id,
                timestamp,
                ..
            } if event_patron_id.0 == patron_id => {
                active_loans.insert(item_id.0.clone(), timestamp);
            }
            librery_core::CirculationEvent::ItemReturned {
                item_id,
                patron_id: event_patron_id,
                ..
            } if event_patron_id.0 == patron_id => {
                active_loans.remove(&item_id.0);
            }
            librery_core::CirculationEvent::HoldFulfilled {
                item_id,
                patron_id: event_patron_id,
                timestamp,
                ..
            } if event_patron_id.0 == patron_id => {
                active_loans.insert(item_id.0.clone(), timestamp);
            }
            _ => {}
        }
    }

    let loans: Vec<LoanResponse> = active_loans
        .into_iter()
        .map(|(item_id, checked_out_at)| LoanResponse {
            item_id,
            checked_out_at,
        })
        .collect();

    (StatusCode::OK, Json(LoansResponse { loans })).into_response()
}

#[utoipa::path(
    get,
    path = "/api/v1/patrons/{id}/holds",
    responses(
        (status = OK, description = "List of active holds for the patron", body = HoldsResponse),
        (status = UNAUTHORIZED, description = "Not authenticated"),
        (status = FORBIDDEN, description = "Not authorized to view this patron's holds"),
        (status = INTERNAL_SERVER_ERROR, description = "Server error")
    ),
    tag = "patrons"
)]
pub async fn get_patron_holds(
    State(state): State<AppState>,
    Path(patron_id): Path<String>,
    axum::Extension(auth): axum::Extension<AuthenticatedPatron>,
) -> axum::response::Response {
    if auth.patron_id != patron_id {
        return StatusCode::FORBIDDEN.into_response();
    }

    let events = match state.circulation_repo.get_events().await {
        Ok(events) => events,
        Err(_) => {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let mut active_holds: std::collections::HashMap<String, i64> = std::collections::HashMap::new();

    for event in events {
        match event {
            librery_core::CirculationEvent::HoldPlaced {
                item_id,
                patron_id: event_patron_id,
                timestamp,
                ..
            } if event_patron_id.0 == patron_id => {
                active_holds.insert(item_id.0.clone(), timestamp);
            }
            librery_core::CirculationEvent::HoldCancelled {
                item_id,
                ..
            } => {
                active_holds.remove(&item_id.0);
            }
            librery_core::CirculationEvent::HoldFulfilled {
                item_id,
                patron_id: event_patron_id,
                ..
            } if event_patron_id.0 == patron_id => {
                active_holds.remove(&item_id.0);
            }
            _ => {}
        }
    }

    let holds: Vec<HoldResponse> = active_holds
        .into_iter()
        .map(|(item_id, placed_at)| HoldResponse {
            item_id,
            placed_at,
        })
        .collect();

    (StatusCode::OK, Json(HoldsResponse { holds })).into_response()
}
