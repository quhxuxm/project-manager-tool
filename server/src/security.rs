use crate::config::CONFIG;
use crate::entity::RoleName;
use crate::error::{Error, SecurityError};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tracing::error;
#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityInfo {
    pub username: String,
    pub roles: Vec<RoleName>,
    pub exp: i64,
    pub organization: Option<String>,
}

impl<S> FromRequestParts<S> for SecurityInfo
where
    S: Send + Sync + 'static,
{
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|e| {
                error!(error = %e, "Failed to extract TypedHeader.");
                Error::Security(SecurityError::AuthorizationHeader)
            })?;
        let token_data = decode::<SecurityInfo>(
            bearer.token(),
            &DecodingKey::from_secret(CONFIG.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| {
            error!(error = %e, "Failed to decode bearer token.");
            Error::Security(SecurityError::InvalidToken(bearer.token().to_string()))
        })?;
        Ok(token_data.claims)
    }
}
