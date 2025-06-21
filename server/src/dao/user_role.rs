use crate::entity::{CreateUserRoleEntity, UserRoleEntity};
use crate::error::Error;
use chrono::Utc;
use sqlx::{Database, Executor, MySql};
use std::marker::PhantomData;
pub struct UserRoleDao<T>
where
    T: Database,
{
    _marker: PhantomData<T>,
}

impl UserRoleDao<MySql> {
    pub async fn save<'c, T>(
        executor: T,
        user_role: CreateUserRoleEntity,
    ) -> Result<UserRoleEntity, Error>
    where
        T: Executor<'c, Database = MySql>,
    {
        let create_date = Utc::now();
        sqlx::query::<MySql>(
            r#"INSERT INTO tb_user_roles (user_id, role_id, create_date) VALUES (?,?,?)"#,
        )
        .bind(user_role.user_id)
        .bind(user_role.role_id)
        .bind(create_date)
        .execute(executor)
        .await?;
        Ok(UserRoleEntity {
            user_id: user_role.user_id,
            role_id: user_role.role_id,
            create_date,
        })
    }
}
