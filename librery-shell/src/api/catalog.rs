use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

use crate::AppState;

#[derive(Serialize, ToSchema)]
pub struct CatalogDetailResponse {
    pub id: String,
    pub title: String,
    pub author: String,
    pub isbn: Option<String>,
    pub publisher: Option<String>,
    pub publication_year: Option<i32>,
    pub availability: Vec<ItemAvailability>,
}

#[derive(Serialize, ToSchema)]
pub struct ItemAvailability {
    pub item_id: String,
    pub barcode: String,
    pub status: String,
    pub location: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/catalog/{id}",
    responses(
        (status = OK, description = "Catalog detail with availability", body = CatalogDetailResponse),
        (status = NOT_FOUND, description = "Bibliographic record not found"),
        (status = INTERNAL_SERVER_ERROR, description = "Server error")
    ),
    tag = "catalog"
)]
pub async fn get_catalog_detail(
    State(state): State<AppState>,
    Path(bib_id): Path<String>,
) -> axum::response::Response {
    let bib_record = match state.bib_repo.find_by_id(&librery_core::BibRecordId(bib_id.clone())).await {
        Ok(Some(record)) => record,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let items = match state.item_repo.find_all().await {
        Ok(items) => items,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let availability: Vec<ItemAvailability> = items
        .into_iter()
        .filter(|item| item.bib_id == bib_id)
        .map(|item| ItemAvailability {
            item_id: item.id.0,
            barcode: item.barcode,
            status: format!("{:?}", item.status),
            location: item.location.0,
        })
        .collect();

    let response = CatalogDetailResponse {
        id: bib_record.id.0,
        title: bib_record.title,
        author: bib_record.author,
        isbn: bib_record.isbn,
        publisher: bib_record.publisher,
        publication_year: bib_record.publication_year,
        availability,
    };

    (StatusCode::OK, Json(response)).into_response()
}
