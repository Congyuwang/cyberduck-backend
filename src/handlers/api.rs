use crate::db_api::public::user_info;
use crate::wechat_login::CodeResponse;
use crate::{DB, SERVER_CONFIG};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::Json;
use axum_database_sessions::{AxumRedisPool, AxumSession};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};
use url::Url;

const WECHAT_ID_KEY: &str = "wechat_openid";
const LOGIN_STATE_KEY: &str = "login_state";
const DUCK_COUNT_THRESHOLD: usize = 10;

pub type Session = AxumSession<AxumRedisPool>;

#[derive(Deserialize, Serialize)]
pub struct LoginState {
    state: String,
    redirect_url: Url,
}

#[derive(Deserialize)]
pub struct LoginParams {
    redirect_url: Url,
}

/// GET login?redirect_url=REDIRECT_URL
pub async fn login(session: Session, Query(login_params): Query<LoginParams>) -> impl IntoResponse {
    if check_login(&session).await.is_none() {
        // already logged in
        let (state, redirect) = SERVER_CONFIG.wechat.auth_url();
        session.set(
            LOGIN_STATE_KEY,
            LoginState {
                state,
                redirect_url: login_params.redirect_url,
            },
        );
        Redirect::to(redirect.as_str())
    } else {
        Redirect::to(login_params.redirect_url.as_str())
    }
}

#[derive(Deserialize)]
pub struct LoginCallbackParams {
    code: String,
    state: String,
}

/// GET login/callback?code=CODE&state=STATE
pub async fn login_callback(
    session: Session,
    Query(login_callback_params): Query<LoginCallbackParams>,
) -> Response {
    if let Some(state) = session.get::<LoginState>(LOGIN_STATE_KEY) {
        session.remove(LOGIN_STATE_KEY);
        if login_callback_params.state.eq(&state.state) {
            match SERVER_CONFIG
                .wechat
                .request_id(&login_callback_params.code)
                .await
            {
                Ok(rsp) => match rsp {
                    CodeResponse::Success { openid, .. } => {
                        session.set(WECHAT_ID_KEY, &openid);
                        info!("login success from openid: {}", openid);
                        Redirect::to(state.redirect_url.as_str()).into_response()
                    }
                    CodeResponse::Failure { errcode, errmsg } => {
                        error!(
                            "error from wechat sns server: (code: {}, msg: {})",
                            errcode, errmsg,
                        );
                        (StatusCode::BAD_REQUEST, "invalid login state").into_response()
                    }
                },
                Err(e) => {
                    error!("error requesting access code: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "error requesting access code",
                    )
                        .into_response()
                }
            }
        } else {
            error!(
                "unmatched state during login callback: (cookie_state={}, callback_state={})",
                state.state, login_callback_params.state
            );
            (StatusCode::BAD_REQUEST, "invalid login state").into_response()
        }
    } else {
        error!("login state not found during login callback");
        (StatusCode::BAD_REQUEST, "invalid login state").into_response()
    }
}

/// GET api/user-info
pub async fn user_info(session: Session, State(db): State<DB>) -> Response {
    if let Some(wechat_openid) = check_login(&session).await {
        match db.upsert_user_info(wechat_openid.clone()).await {
            Ok(data) => {
                info!("user info request success: openid={}", wechat_openid);
                Json(data).into_response()
            }
            Err(e) => {
                error!("error getting or creating user: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "error getting or creating user",
                )
                    .into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "please login first").into_response()
    }
}

/// DELETE api/user-info
pub async fn clear_history(session: Session, State(db): State<DB>) -> Response {
    if let Some(wechat_openid) = check_login(&session).await {
        match db.delete_duck_history(wechat_openid).await {
            Ok(n) => Json(json!({ "number_of_records_removed": n })).into_response(),
            Err(e) => {
                error!("error removing game history: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "error removing game history",
                )
                    .into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "please login first").into_response()
    }
}

/// GET api/find-duck/:duck_id
pub async fn find_duck(
    session: Session,
    State(db): State<DB>,
    Path(duck_id): Path<String>,
) -> Response {
    if let Some(wechat_openid) = check_login(&session).await {
        match db
            .record_duck_view(wechat_openid.clone(), duck_id.clone())
            .await
        {
            Ok(mut data) => {
                info!(
                    "user (openid: {}) find duck (duck_id: {})",
                    wechat_openid, duck_id
                );
                if data.duck_history.len() == DUCK_COUNT_THRESHOLD {
                    match db.upsert_ranking(wechat_openid).await {
                        Err(e) => {
                            error!("error recording ranking: {}", e);
                            return (StatusCode::INTERNAL_SERVER_ERROR, "error recording ranking")
                                .into_response();
                        }
                        // add ranking info
                        Ok(history) => {
                            data.ranking = Some(user_info::ranking::Data {
                                ranking: history.ranking,
                            });
                        }
                    }
                }
                Json(data).into_response()
            }
            Err(e) => {
                error!("error recording duck view: {}", e);
                (StatusCode::NOT_FOUND, "error recording duck view").into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, "please login first").into_response()
    }
}

/// GET api/preview-ducks
pub async fn preview_ducks(State(db): State<DB>) -> Response {
    match db.preview_ducks().await {
        Ok(data) => Json(data).into_response(),
        Err(e) => {
            error!("error previewing ducks: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "error previewing ducks").into_response()
        }
    }
}

async fn check_login(session: &Session) -> Option<String> {
    session.get::<String>(WECHAT_ID_KEY)
}
