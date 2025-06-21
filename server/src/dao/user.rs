use crate::entity::{CreateUserEntity, UserEntity};
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
}
