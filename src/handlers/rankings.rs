use crate::{IntoResponse, Response, DB, SERVER_CONFIG};
use axum::extract::State;
use axum::Json;
use axum_auth::AuthBearer;
use http::StatusCode;
use serde_json::json;
use tracing::error;

/// GET admin/rankings
pub async fn get_all_rankings(AuthBearer(token): AuthBearer, State(db): State<DB>) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.get_all_rankings().await {
            Ok(rsp) => Json(rsp).into_response(),
            Err(e) => {
                error!("error getting rankings: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "error getting rankings").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}

/// GET admin/rankings
pub async fn delete_all_rankings(AuthBearer(token): AuthBearer, State(db): State<DB>) -> Response {
    if token.eq(&SERVER_CONFIG.admin_token) {
        match db.delete_all_rankings().await {
            Ok(rsp) => Json(json!({
                "number_of_rankings_deleted": rsp,
            }))
            .into_response(),
            Err(e) => {
                error!("error deleting rankings: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "error deleting rankings").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "provide admin token").into_response()
    }
}
