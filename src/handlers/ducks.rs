use crate::db_api::ducks::{NewDuckData, UpdateDuckData};
use crate::{DB, SERVER_CONFIG};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use axum_auth::AuthBearer;
use serde::Deserialize;
use serde_json::json;
use tracing::error;

/// POST admin/duck
pub async fn create_duck(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
    Json(data): Json<NewDuckData>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.create_duck(data).await {
            Ok(rsp) => Json(rsp).into_response(),
            Err(e) => {
                error!("error creating duck: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "error creating duck").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}

/// GET admin/duck/:id
pub async fn get_duck(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
    Path(duck_id): Path<String>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.get_duck(duck_id).await {
            Ok(Some(rsp)) => Json(rsp).into_response(),
            Ok(None) => (StatusCode::NOT_FOUND, "duck id does not exist").into_response(),
            Err(e) => {
                error!("error getting duck: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "error getting duck").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}

/// PATCH admin/duck/:id
pub async fn update_duck(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
    Path(duck_id): Path<String>,
    Json(data): Json<UpdateDuckData>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.update_duck(duck_id, data).await {
            Ok(rsp) => Json(rsp).into_response(),
            Err(e) => {
                error!("error updating duck: {}", e);
                (StatusCode::BAD_REQUEST, "error updating duck").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}

/// DELETE admin/duck/:id
pub async fn delete_duck(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
    Path(duck_id): Path<String>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.delete_duck(duck_id).await {
            Ok(rsp) => Json(rsp).into_response(),
            Err(e) => {
                error!("error deleting duck: {}", e);
                (StatusCode::BAD_REQUEST, "error deleting duck").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}

/// POST admin/many-ducks
pub async fn create_many_ducks(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
    Json(data): Json<Vec<NewDuckData>>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.create_many_ducks(data).await {
            Ok(rsp) => Json(json!({
                "number_of_ducks_created": rsp,
            }))
            .into_response(),
            Err(e) => {
                error!("error creating ducks: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "error creating ducks").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}

/// GET admin/many-ducks
pub async fn get_all_ducks(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.get_all_ducks().await {
            Ok(rsp) => Json(rsp).into_response(),
            Err(e) => {
                error!("error getting ducks: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "error getting ducks").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}

#[derive(Deserialize)]
pub struct DeleteDuckHistoryParam {
    user_id: String,
}

/// DELETE admin/duck-history/dangerous?user_id=USER_ID
pub async fn delete_duck_history(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
    Query(params): Query<DeleteDuckHistoryParam>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.delete_duck_history(params.user_id).await {
            Ok(rsp) => Json(json!({
                "number_of_duck_view_records_deleted": rsp,
            }))
            .into_response(),
            Err(e) => {
                error!("error deleting duck history: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "error deleting ducks history",
                )
                    .into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}

/// DELETE admin/dangerous/many-ducks
pub async fn delete_all_ducks(
    AuthBearer(token): AuthBearer,
    Extension(db): Extension<DB>,
) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.delete_all_ducks().await {
            Ok(rsp) => Json(json!({
                "number_of_ducks_deleted": rsp,
            }))
            .into_response(),
            Err(e) => {
                error!("error deleting ducks: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "error deleting ducks").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}
