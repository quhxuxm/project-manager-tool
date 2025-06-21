use crate::entity::{CreateRoleEntity, RoleEntity};
use crate::error::Error;
use chrono::Utc;
use sqlx::{Database, Executor, MySql, QueryBuilder};
use std::marker::PhantomData;
pub struct RoleDao<T>
where
    T: Database,
{
    _marker: PhantomData<T>,
}

impl RoleDao<MySql> {
    pub async fn save<'c, T>(executor: T, role: CreateRoleEntity) -> Result<RoleEntity, Error>
    where
        T: Executor<'c, Database = MySql>,
    {
        let create_date = Utc::now();
        let insert_result = sqlx::query::<MySql>(
            r#"
                INSERT INTO tb_roles (name, description, create_date) 
                VALUES (?,?,?);
                "#,
        )
        .bind(&role.name)
        .bind(&role.description)
        .bind(create_date)
        .execute(executor)
        .await?;
        Ok(RoleEntity {
            id: insert_result.last_insert_id(),
            name: role.name,
            description: role.description,
            create_date,
        })
    }

    pub async fn find<'c, 'a, T, N>(executor: T, names: N) -> Result<Vec<RoleEntity>, Error>
    where
        T: Executor<'c, Database = MySql>,
        N: Iterator<Item = &'a str>,
    {
        let mut roles_sql_builder =
            QueryBuilder::<MySql>::new("SELECT * FROM tb_roles WHERE name in");
        roles_sql_builder.push_tuples(names, |mut bind, item| {
            bind.push_bind(item);
        });
        let roles_sql = roles_sql_builder.build_query_as::<RoleEntity>();
        let roles = roles_sql.fetch_all(executor).await?;
        Ok(roles)
    }
}
