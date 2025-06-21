use crate::dao::role::RoleDao;
use crate::dao::user::UserDao;
use crate::dao::user_role::UserRoleDao;
use crate::entity::{CreateUserEntity, CreateUserRoleEntity};
use crate::error::Error;
use crate::request::CreateUserRequest;
use crate::response::{CreateUserResponse, FindUserResponse};
use crate::state::ServerState;
use axum::extract::Path;
use axum::{Extension, Json};
use axum_macros::debug_handler;
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
    let roles = RoleDao::find_by_names(
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

#[debug_handler]
pub async fn find_user(
    Extension(server_state): Extension<Arc<ServerState>>,
    Path(username): Path<String>,
) -> Result<Json<FindUserResponse>, Error> {
    let user_with_role_name_entities =
        UserDao::find_with_role_name_by_username(&server_state.connection_pool, &username).await?;
    let first_user = user_with_role_name_entities
        .first()
        .ok_or(Error::UserNotFound(username.to_string()))?;
    let user_id = first_user.id;
    let username = first_user.username.clone();
    let create_date = first_user.create_date;
    let mut roles = Vec::new();
    user_with_role_name_entities
        .into_iter()
        .for_each(|role| roles.push(role.role_name));
    let response = FindUserResponse {
        id: user_id,
        username,
        create_date,
        roles,
    };
    Ok(Json(response))
}
