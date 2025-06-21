mod dao;
mod entity;
mod error;
mod handler;
mod request;
mod response;
mod state;
use crate::state::ServerState;
use anyhow::Result;
use axum::routing::post;
use axum::{Extension, Router};
use sqlx::mysql::MySqlPoolOptions;
use std::sync::Arc;
#[tokio::main]
async fn main() -> Result<()> {
    let connection_pool = MySqlPoolOptions::new()
        .max_connections(32)
        .connect("mysql://root:password@localhost:3306/project_manage_tool")
        .await?;
    let app = Router::new()
        .route("/roles", post(handler::create_role))
        .route("/users", post(handler::create_user))
        .route("/projects", post(handler::create_project))
        .route("/stories", post(handler::create_story))
        .layer(Extension(Arc::new(ServerState { connection_pool })));

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(tcp_listener, app).await?;
    Ok(())
}
