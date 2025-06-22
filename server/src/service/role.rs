use crate::dao::role::RoleDao;
use crate::entity::CreateRoleEntity;
use crate::error::Error;
use crate::request::CreateRoleRequest;
use crate::response::CreateRoleResponse;
use sqlx::{MySql, MySqlPool};
pub struct RoleService;

impl RoleService {
    pub async fn save(
        connection_pool: &MySqlPool,
        create_role_request: CreateRoleRequest,
    ) -> Result<CreateRoleResponse, Error> {
        let create_role = CreateRoleEntity {
            name: create_role_request.name,
            description: create_role_request.description,
        };
        let role_entity = RoleDao::<MySql>::save(connection_pool, create_role).await?;
        Ok(CreateRoleResponse {
            id: role_entity.id,
            name: role_entity.name,
            description: role_entity.description,
            create_date: role_entity.create_date,
        })
    }
}
