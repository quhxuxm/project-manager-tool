use crate::dao::role::RoleDao;
use crate::dao::user::UserDao;
use crate::dao::user_role::UserRoleDao;
use crate::entity::{CreateUserEntity, CreateUserRoleEntity};
use crate::error::Error;
use crate::request::CreateUserRequest;
use crate::response::CreateUserResponse;
use crate::state::ServerState;
use axum::{Extension, Json};
use std::sync::Arc;
#[axum_macros::debug_handler]
pub async fn create_user(
    Extension(server_state): Extension<Arc<ServerState>>,
    Json(create_user_request): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, Error> {
    let mut txn = server_state.connection_pool.begin().await?;
    let user = CreateUserEntity {
        username: create_user_request.username,
        password: create_user_request.password,
    };
    let user = UserDao::save(&mut *txn, user).await?;
    let roles = RoleDao::find(
        &mut *txn,
        create_user_request.roles.iter().map(|item| item.as_str()),
    )
    .await?;
    let mut roles_in_response = Vec::new();
    for role in roles.into_iter() {
        let create_user_role = CreateUserRoleEntity {
            user_id: user.id,
            role_id: role.id,
        };
        UserRoleDao::save(&mut *txn, create_user_role).await?;
        roles_in_response.push(role.name);
    }
    txn.commit().await?;
    Ok(Json(CreateUserResponse {
        id: user.id,
        username: user.username,
        create_date: user.create_date,
        roles: roles_in_response,
    }))
}
