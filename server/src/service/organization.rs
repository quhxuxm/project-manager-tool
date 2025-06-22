use crate::dao::organization::OrganizationDao;
use crate::entity::{CreateOrganizationEntity, RoleName};
use crate::error::{Error, SecurityError};
use crate::request::CreateOrganizationRequest;
use crate::response::CreateOrganizationResponse;
use crate::security::SecurityInfo;
use crate::service::UserService;
use sqlx::MySqlPool;
pub struct OrganizationService;

impl OrganizationService {
    pub async fn save(
        connection_pool: &MySqlPool,
        security_info: &SecurityInfo,
        create_organization_request: CreateOrganizationRequest,
    ) -> Result<CreateOrganizationResponse, Error> {
        let user =
            UserService::find_user_and_role(connection_pool, &security_info.username).await?;
        if !user.roles.contains(&RoleName::OrgAdmin) {
            return Err(Error::Security(SecurityError::RequireAuthorization(
                RoleName::OrgAdmin,
            )));
        }
        let create_org_entity = CreateOrganizationEntity {
            name: create_organization_request.name,
            description: create_organization_request.description,
            creator_id: user.id,
        };
        let org_entity = OrganizationDao::save(connection_pool, create_org_entity).await?;
        Ok(CreateOrganizationResponse {
            name: org_entity.name,
            description: org_entity.description,
            create_date: org_entity.create_date,
        })
    }
}
