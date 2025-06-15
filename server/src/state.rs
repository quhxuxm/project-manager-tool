use sqlx::{Pool, Sqlite};
pub struct ServerState {
    pub db_connection_pool: Pool<Sqlite>,
}
