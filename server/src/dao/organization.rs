use crate::entity::{CreateOrganizationEntity, OrganizationEntity};
use crate::error::Error;
use chrono::Utc;
use sqlx::{Database, Executor, MySql};
use std::marker::PhantomData;
pub struct OrganizationDao<T>
where
    T: Database,
{
    _mark: PhantomData<T>,
}

impl OrganizationDao<MySql> {
    pub async fn find_by_user<'c, E>(
        connection_pool: E,
        user_id: u64,
    ) -> Result<Vec<OrganizationEntity>, Error>
    where
        E: Executor<'c, Database = MySql>,
    {
        let org_entities = sqlx::query_as::<MySql, OrganizationEntity>(
            r#"SELECT * FROM tb_organizations WHERE creator_id = ?"#,
        )
        .bind(user_id)
        .fetch_all(connection_pool)
        .await?;
        Ok(org_entities)
    }

    pub async fn save<'c, E>(
        executor: E,
        organization: CreateOrganizationEntity,
    ) -> Result<OrganizationEntity, Error>
    where
        E: Executor<'c, Database = MySql>,
    {
        let create_date = Utc::now();
        let insert_result = sqlx::query::<MySql>(
            r#"
            INSERT INTO tb_organizations (
                name, description, create_date, creator_id
            ) VALUES (
                ?,?,?,?
            );
        "#,
        )
        .bind(&organization.name)
        .bind(&organization.description)
        .bind(&create_date)
        .bind(&organization.creator_id)
        .execute(executor)
        .await?;
        Ok(OrganizationEntity {
            id: insert_result.last_insert_id(),
            name: organization.name,
            description: organization.description,
            create_date,
        })
    }
}
