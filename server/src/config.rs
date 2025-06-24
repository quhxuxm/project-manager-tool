use serde::{Deserialize, Serialize};
use std::io::Read;
use std::net::SocketAddr;
use std::sync::LazyLock;
static DEFAULT_CONFIG_FILE: &str = "server-config.toml";
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let mut config_file =
        std::fs::File::open(DEFAULT_CONFIG_FILE).expect("Failed to open config file");
    let mut config_file_content = String::new();
    config_file
        .read_to_string(&mut config_file_content)
        .expect("Failed to read config file");
    let config =
        toml::from_str::<Config>(&config_file_content).expect("Failed to parse config file");
    config
});
#[derive(Serialize, Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub database_connection_pool_max_size: u32,
    pub database_connection_pool_min_size: u32,
    pub database_test_before_acquire: bool,
    pub listening_address: SocketAddr,
    pub jwt_secret: String,
    pub jwt_expiration_time: i64,
}
