use crate::config::CONFIG;
use crate::error::{Error, SecurityError};
use crate::request::{AuthenticateRequest, CreateUserRequest};
use crate::response::{AuthenticateResponse, CreateUserResponse, FindUserResponse};
use crate::security::SecurityInfo;
use crate::service::UserService;
use crate::state::ServerState;
use axum::extract::Path;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use chrono::{TimeDelta, Timelike, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::ops::Add;
use std::sync::Arc;
use tracing::debug;
#[debug_handler]
pub async fn create_user(
    Extension(server_state): Extension<Arc<ServerState>>,
    Json(create_user_request): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, Error> {
    let create_user_response =
        UserService::save(&server_state.connection_pool, create_user_request).await?;
    Ok(Json(create_user_response))
}
#[debug_handler]
pub async fn find_user(
    Extension(server_state): Extension<Arc<ServerState>>,
    Path(username): Path<String>,
) -> Result<Json<FindUserResponse>, Error> {
    let response =
        UserService::find_user_and_role(&server_state.connection_pool, &username).await?;
    Ok(Json(response))
}

#[debug_handler]
pub async fn authenticate(
    Extension(server_state): Extension<Arc<ServerState>>,
    Json(AuthenticateRequest { username, password }): Json<AuthenticateRequest>,
) -> Result<Json<AuthenticateResponse>, Error> {
    let find_user_response =
        UserService::find_user_and_role(&server_state.connection_pool, &username).await?;
    if find_user_response.password != password {
        return Err(Error::Security(SecurityError::Authentication));
    }
    let expired_at = Utc::now();
    let expired_at = expired_at.add(TimeDelta::seconds(CONFIG.jwt_expiration_time));
    debug!("Security info expired at: {:?}", expired_at);
    let security_info = SecurityInfo {
        username,
        roles: find_user_response.roles,
        exp: expired_at.timestamp(),
        organization: None,
    };
    debug!("Generated security info: {:?}", security_info);
    let authentication_token = encode(
        &Header::default(),
        &security_info,
        &EncodingKey::from_secret(CONFIG.jwt_secret.as_bytes()),
    )
    .map_err(|e| Error::Security(SecurityError::GenerateToken))?;
    Ok(Json(AuthenticateResponse {
        authentication_token,
    }))
}
