use crate::db_api::exhibits::{NewExhibitData, UpdateExhibitData};
use crate::{DB, SERVER_CONFIG};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use axum_auth::AuthBearer;
use serde_json::json;
use tracing::error;

/// POST admin/exhibit
pub async fn create_exhibit(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
    Json(data): Json<NewExhibitData>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.create_exhibit(data).await {
            Ok(rsp) => Json(rsp).into_response(),
            Err(e) => {
                error!("error creating exhibit: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "error creating exhibit").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}

/// GET admin/exhibit/:id
pub async fn get_exhibit(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
    Path(exhibit_id): Path<String>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.get_exhibit(exhibit_id).await {
            Ok(Some(rsp)) => Json(rsp).into_response(),
            Ok(None) => (StatusCode::NOT_FOUND, "exhibit id does not exist").into_response(),
            Err(e) => {
                error!("error getting exhibit: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "error getting exhibit").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}

/// PATCH admin/exhibit/:id
pub async fn update_exhibit(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
    Path(exhibit_id): Path<String>,
    Json(data): Json<UpdateExhibitData>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.update_exhibit(exhibit_id, data).await {
            Ok(rsp) => Json(rsp).into_response(),
            Err(e) => {
                error!("error updating exhibit: {}", e);
                (StatusCode::NOT_FOUND, "error updating exhibit").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}

/// DELETE admin/exhibit/:id
pub async fn delete_exhibit(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
    Path(exhibit_id): Path<String>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.delete_exhibit(exhibit_id).await {
            Ok(rsp) => Json(rsp).into_response(),
            Err(e) => {
                error!("error deleting exhibit: {}", e);
                (StatusCode::NOT_FOUND, "error deleting exhibit").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}

/// POST admin/many-exhibits
pub async fn create_many_exhibits(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
    Json(data): Json<Vec<NewExhibitData>>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.create_many_exhibits(data).await {
            Ok(rsp) => Json(json!({
                "number_of_exhibits_created": rsp,
            }))
            .into_response(),
            Err(e) => {
                error!("error creating exhibits: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "error creating exhibits").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}

/// GET admin/many-exhibits
pub async fn get_all_exhibits(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.get_all_exhibits().await {
            Ok(rsp) => Json(rsp).into_response(),
            Err(e) => {
                error!("error getting exhibits: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "error getting exhibits").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}

/// DELETE admin/dangerous/many-exhibits
pub async fn delete_all_exhibits(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.delete_all_exhibits().await {
            Ok(rsp) => Json(json!({
                "number_of_exhibits_deleted": rsp,
            }))
            .into_response(),
            Err(e) => {
                error!("error deleting exhibits: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "error deleting exhibits").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}
