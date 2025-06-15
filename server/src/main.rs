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
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;
#[tokio::main]
async fn main() -> Result<()> {
    let db_connection_pool = SqlitePoolOptions::new()
        .max_connections(32)
        .connect("file://pmt.db")
        .await?;
    let app = Router::new()
        // `POST /users` goes to `create_user`
        .route("/managers", post(handler::create_manager))
        .route("/engineers", post(handler::create_engineer))
        .route("/projects", post(handler::create_project))
        .route("/stories", post(handler::create_story))
        .layer(Extension(Arc::new(ServerState { db_connection_pool })));

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(tcp_listener, app).await?;
    Ok(())
}
