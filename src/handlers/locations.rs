use crate::db_api::locations::{NewLocationData, UpdateLocationData};
use crate::{DB, SERVER_CONFIG};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use axum_auth::AuthBearer;
use serde_json::json;
use tracing::error;

/// POST admin/location
pub async fn create_location(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
    Json(data): Json<NewLocationData>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.create_location(data).await {
            Ok(rsp) => Json(rsp).into_response(),
            Err(e) => {
                error!("error creating location: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "error creating location").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}

/// GET admin/location/:id
pub async fn get_location(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
    Path(location_id): Path<String>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.get_location(location_id).await {
            Ok(Some(rsp)) => Json(rsp).into_response(),
            Ok(None) => (StatusCode::NOT_FOUND, "location id does not exist").into_response(),
            Err(e) => {
                error!("error getting location: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "error getting location").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}

/// PATCH admin/location/:id
pub async fn update_location(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
    Path(location_id): Path<String>,
    Json(data): Json<UpdateLocationData>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.update_location(location_id, data).await {
            Ok(rsp) => Json(rsp).into_response(),
            Err(e) => {
                error!("error updating location: {}", e);
                (StatusCode::NOT_FOUND, "error updating location").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}

/// DELETE admin/location/:id
pub async fn delete_location(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
    Path(location_id): Path<String>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.delete_location(location_id).await {
            Ok(rsp) => Json(rsp).into_response(),
            Err(e) => {
                error!("error deleting location: {}", e);
                (StatusCode::NOT_FOUND, "error deleting location").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}

/// POST admin/many-locations
pub async fn create_many_locations(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
    Json(data): Json<Vec<NewLocationData>>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.create_many_locations(data).await {
            Ok(rsp) => Json(json!({
                "number_of_locations_created": rsp,
            }))
            .into_response(),
            Err(e) => {
                error!("error creating locations: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "error creating locations",
                )
                    .into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}

/// GET admin/many-locations
pub async fn get_all_locations(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.get_all_locations().await {
            Ok(rsp) => Json(rsp).into_response(),
            Err(e) => {
                error!("error getting locations: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "error getting locations").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}

/// DELETE admin/dangerous/many-locations
pub async fn delete_all_locations(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.delete_all_locations().await {
            Ok(rsp) => Json(json!({
                "number_of_locations_deleted": rsp,
            }))
            .into_response(),
            Err(e) => {
                error!("error deleting locations: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "error deleting locations",
                )
                    .into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}
