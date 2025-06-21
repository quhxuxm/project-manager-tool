use sqlx::{MySql, Pool};
pub struct ServerState {
    pub connection_pool: Pool<MySql>,
}
