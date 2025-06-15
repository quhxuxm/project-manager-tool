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
    let mut db_connection = server_state.db_connection_pool.acquire().await?;
    db_connection.begin().await?;
    let user = User {
        id: 0,
        username: "".to_string(),
        password: "".to_string(),
        create_date: Default::default(),
        role: Role::Manager,
    };
    sqlx::query_with::<_, User>("select * from manager", user)
        .execute(&mut db_connection)
        .await?;
    todo!()
}
