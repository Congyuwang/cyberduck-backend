mod configuration;
mod db_api;
mod handlers;
mod prisma;
mod redis_session_layer;
mod wechat_login;

use crate::db_api::DB;
use crate::handlers::{api, ducks, exhibits};
use anyhow::Result;
use axum::routing::{delete, get, post};
use axum::{Extension, Router};
use axum_server::tls_rustls::RustlsConfig;
use configuration::Configuration;
use std::fs::OpenOptions;
use std::net::SocketAddr;
use tracing_subscriber::prelude::*;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref SERVER_CONFIG: Configuration = Configuration::load().unwrap();
}

#[tokio::main]
async fn main() -> Result<()> {
    // configure log
    let stdout_log = tracing_subscriber::fmt::layer().pretty();
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&SERVER_CONFIG.log_file)?;
    let log_to_file = tracing_subscriber::fmt::layer().with_writer(log_file);
    // tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "wechat_login=info".into()),
        ))
        .with(stdout_log)
        .with(log_to_file)
        .init();

    // mongodb
    let db = DB::new(&SERVER_CONFIG.db_url).await?;

    // redis session
    let session = SERVER_CONFIG.redis_session.build_layer().await?;

    // routers
    let admin = Router::new()
        .route("/duck", post(ducks::create_duck))
        .route(
            "/duck/:id",
            get(ducks::get_duck)
                .patch(ducks::update_duck)
                .delete(ducks::delete_duck),
        )
        .route(
            "/many-ducks",
            get(ducks::get_all_ducks).post(ducks::create_many_ducks),
        )
        .route("/many-ducks/dangerous", delete(ducks::delete_all_ducks))
        .route("/exhibit", post(exhibits::create_exhibit))
        .route(
            "/exhibit/:id",
            get(exhibits::get_exhibit)
                .patch(exhibits::update_exhibit)
                .delete(exhibits::delete_exhibit),
        )
        .route(
            "/many-exhibits",
            get(exhibits::get_all_exhibits).post(exhibits::create_many_exhibits),
        )
        .route(
            "/many-exhibits/dangerous",
            delete(exhibits::delete_all_exhibits),
        );

    let api = Router::new()
        .route("/user-info", get(api::user_info))
        .route("/find-duck/:duck_id", get(api::find_duck));

    let callback_path = &SERVER_CONFIG.wechat.redirect_uri.path();
    let app = Router::new()
        .nest("/admin", admin)
        .nest("/api", api)
        .route("/login", get(api::login))
        .route(callback_path, get(api::login_callback))
        .layer(Extension(db))
        .layer(session);

    // start listening
    let addr: SocketAddr = SERVER_CONFIG.server_binding.parse()?;
    tracing::info!("server listening at {}", addr);
    if let Some(tls) = &SERVER_CONFIG.server_tls {
        tracing::info!("serving with secure connection");
        let tls_config = RustlsConfig::from_pem_file(&tls.cert, &tls.key).await?;
        axum_server::bind_rustls(addr, tls_config)
            .serve(app.into_make_service())
            .await?
    } else {
        axum_server::bind(addr)
            .serve(app.into_make_service())
            .await?
    };
    Ok(())
}
