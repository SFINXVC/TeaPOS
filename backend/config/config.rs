use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server_address: String,
    pub server_port: u16,

    pub database_url: String,
    pub database_pool_size: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_address: "127.0.0.1".to_string(),
            server_port: 3000,
            database_url: "postgres://teapos:123456@localhost:5432/teapos".to_string(),
            database_pool_size: 10,
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Config, envy::Error> {
        dotenv().ok();

        envy::from_env::<Config>()
    }
}