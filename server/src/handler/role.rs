use crate::error::Error;
use crate::request::CreateRoleRequest;
use crate::response::CreateRoleResponse;
use crate::service::RoleService;
use crate::state::ServerState;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use std::sync::Arc;
#[debug_handler]
pub async fn create_role(
    Extension(server_state): Extension<Arc<ServerState>>,
    Json(create_role_request): Json<CreateRoleRequest>,
) -> Result<Json<CreateRoleResponse>, Error> {
    let create_role_response =
        RoleService::save(&server_state.connection_pool, create_role_request).await?;
    Ok(Json(create_role_response))
}
