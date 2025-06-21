use crate::dao::role::RoleDao;
use crate::entity::CreateRoleEntity;
use crate::error::Error;
use crate::request::CreateRoleRequest;
use crate::response::CreateRuleResponse;
use crate::state::ServerState;
use axum::{Extension, Json};
use sqlx::MySql;
use std::sync::Arc;
#[axum_macros::debug_handler]
pub async fn create_role(
    Extension(server_state): Extension<Arc<ServerState>>,
    Json(create_role_request): Json<CreateRoleRequest>,
) -> Result<Json<CreateRuleResponse>, Error> {
    let create_role = CreateRoleEntity {
        name: create_role_request.name,
        description: create_role_request.description,
    };
    let role_entity = RoleDao::<MySql>::save(&server_state.connection_pool, create_role).await?;
    Ok(Json(CreateRuleResponse {
        id: role_entity.id,
        name: role_entity.name,
        description: role_entity.description,
        create_date: role_entity.create_date,
    }))
}
