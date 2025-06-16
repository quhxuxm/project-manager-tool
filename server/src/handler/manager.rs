use crate::entity::{Role, User};
use crate::error::Error;
use crate::response::CreateManagerResponse;
use crate::state::ServerState;
use axum::{Extension, Json};
use sqlx::Acquire;
use std::sync::Arc;
pub async fn create_manager(
    Extension(server_state): Extension<Arc<ServerState>>,
) -> Result<Json<CreateManagerResponse>, Error> {
    server_state.db_connection_pool.begin().await?;
    let user = User {
        id: 0,
        username: "".to_string(),
        password: "".to_string(),
        create_date: Default::default(),
        role: Role::Manager,
    };
    sqlx::query(r#"INSERT INTO tb_manager (
                                id, username, password, create_date, role
                            ) VALUES (
                                $1, $2, $3, $4, $5
                            )"#)
        .bind(user.id).bind(user.username).bind(user.password).bind(user.create_date).bind(user.role)
        .execute(&server_state.db_connection_pool)
        .await?;
    todo!()
}
