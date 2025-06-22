use crate::error::Error;
use crate::request::CreateOrganizationRequest;
use crate::response::CreateOrganizationResponse;
use crate::security::SecurityInfo;
use crate::service::OrganizationService;
use crate::state::ServerState;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use std::sync::Arc;
use tracing::debug;
#[debug_handler]
pub async fn create_organization(
    Extension(server_state): Extension<Arc<ServerState>>,
    security_info: SecurityInfo,
    Json(create_organization_request): Json<CreateOrganizationRequest>,
) -> Result<Json<CreateOrganizationResponse>, Error> {
    debug!("Receive security info: {:?}", security_info);
    debug!("Create organization: {:?}", create_organization_request);
    let create_org_response = OrganizationService::save(
        &server_state.connection_pool,
        &security_info,
        create_organization_request,
    )
    .await?;
    Ok(Json(create_org_response))
}
