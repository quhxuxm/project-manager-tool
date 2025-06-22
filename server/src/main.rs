mod config;
mod dao;
mod entity;
mod error;
mod handler;
mod request;
mod response;
mod security;
mod service;
mod state;
use crate::config::CONFIG;
use crate::state::ServerState;
use anyhow::{Context, Result};
use axum::routing::{get, post};
use axum::{Extension, Router};
use sqlx::mysql::MySqlPoolOptions;
use std::sync::Arc;
use tracing::{Instrument, Level};
use tracing_subscriber::FmtSubscriber;
#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_line_number(true)
        .with_level(true)
        .with_file(true)
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    let connection_pool = MySqlPoolOptions::new()
        .max_connections(CONFIG.database_connection_pool_max_size)
        .min_connections(CONFIG.database_connection_pool_min_size)
        .test_before_acquire(CONFIG.database_test_before_acquire)
        .connect(&CONFIG.database_url)
        .await
        .context("Creating database connection pool failed")?;
    let app = Router::new()
        .route("/role", post(handler::create_role))
        .route("/user", post(handler::create_user))
        .route("/user/{username}", get(handler::find_user))
        .route("/auth", post(handler::authenticate))
        .route("/org", post(handler::create_organization))
        .layer(Extension(Arc::new(ServerState { connection_pool })));

    let tcp_listener = tokio::net::TcpListener::bind(CONFIG.listening_address).await?;
    axum::serve(tcp_listener, app)
        .await
        .context("Server failed to start")?;
    Ok(())
}
