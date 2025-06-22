use crate::entity::{CreateUserEntity, UserEntity, UserWithRoleNameEntity};
use crate::error::Error;
use chrono::Utc;
use sqlx::{Executor, MySql};
use std::marker::PhantomData;
pub struct UserDao<T> {
    _marker: PhantomData<T>,
}

impl UserDao<MySql> {
    pub async fn save<'c, T>(executor: T, user: CreateUserEntity) -> Result<UserEntity, Error>
    where
        T: Executor<'c, Database = MySql>,
    {
        let create_date = Utc::now();
        let insert_result = sqlx::query::<MySql>(
            r#"
                INSERT INTO tb_users (username, password, create_date)
                VALUES (?, ?, ?)
                "#,
        )
        .bind(&user.username)
        .bind(&user.password)
        .bind(create_date)
        .execute(executor)
        .await?;
        Ok(UserEntity {
            id: insert_result.last_insert_id(),
            username: user.username,
            password: user.password,
            create_date,
        })
    }

    pub async fn find_user<'c, T>(executor: T, username: &str) -> Result<UserEntity, Error>
    where
        T: Executor<'c, Database = MySql>,
    {
        let user =
            sqlx::query_as::<MySql, UserEntity>(r#"SELECT * FROM tb_users WHERE username = ?"#)
                .bind(username)
                .fetch_one(executor)
                .await?;
        Ok(user)
    }

    pub async fn find_user_and_role<'c, T>(
        executor: T,
        username: &str,
    ) -> Result<Vec<UserWithRoleNameEntity>, Error>
    where
        T: Executor<'c, Database = MySql>,
    {
        let user_with_role_name_entities = sqlx::query_as::<MySql, UserWithRoleNameEntity>(
            r#"
                SELECT 
                    u.id,
                    u.username,
                    u.password,
                    u.create_date,
                    r.name as role_name
                FROM 
                    tb_users as u,
                    tb_user_roles as ur,
                    tb_roles as r
                WHERE 
                    u.username = ? AND
                    ur.user_id = u.id AND
                    ur.role_id = r.id;
                "#,
        )
        .bind(username)
        .fetch_all(executor)
        .await?;
        Ok(user_with_role_name_entities)
    }
}
