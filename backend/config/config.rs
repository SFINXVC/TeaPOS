use dotenv::dotenv;
use serde::Deserialize;
use crate::error::Result;

#[derive(Deserialize)]
pub struct Config {
    pub server_address: String,
    pub server_port: u16,

    pub database_url: String,
    pub database_pool_size: u32,

    pub redis_url: String,

    pub acc_token_secret: String,
    pub ref_token_secret: String,
    pub acc_token_expiry: i64,
    pub ref_token_expiry: i64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_address: "127.0.0.1".to_string(),
            server_port: 3000,
            database_url: "postgres://teapos:123456@localhost:5432/teapos".to_string(),
            database_pool_size: 10,
            redis_url: "redis://127.0.0.1:6379".to_string(),
            acc_token_secret: "t34p0s_acc_r1ll_s3cr3t".to_string(),
            ref_token_secret: "t34p0s_ref_r1ll_s3cr3t".to_string(),
            acc_token_expiry: 3600,
            ref_token_expiry: 86400,
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();
        
        let default_config = Self::default();
        
        let server_address = Self::get_env_or_default("SERVER_ADDRESS", default_config.server_address.clone())?;
        let server_port = Self::get_env_or_default("SERVER_PORT", default_config.server_port)?;
        let database_url = Self::get_env_or_default("DATABASE_URL", default_config.database_url.clone())?;
        let database_pool_size = Self::get_env_or_default("DATABASE_POOL_SIZE", default_config.database_pool_size)?;
        let redis_url = Self::get_env_or_default("REDIS_URL", default_config.redis_url.clone())?;
        let acc_token_secret = Self::get_env_or_default("ACC_TOKEN_SECRET", default_config.acc_token_secret.clone())?;
        let ref_token_secret = Self::get_env_or_default("REF_TOKEN_SECRET", default_config.ref_token_secret.clone())?;
        let acc_token_expiry = Self::get_env_or_default("ACC_TOKEN_EXPIRY", default_config.acc_token_expiry)?;
        let ref_token_expiry = Self::get_env_or_default("REF_TOKEN_EXPIRY", default_config.ref_token_expiry)?;
        
        Ok(Self {
            server_address,
            server_port,
            database_url,
            database_pool_size,
            redis_url,
            acc_token_secret,
            ref_token_secret,
            acc_token_expiry,
            ref_token_expiry,
        })
    }
    
    fn get_env_or_default<T: std::str::FromStr + std::fmt::Display>(key: &str, default: T) -> Result<T> 
    where
        <T as std::str::FromStr>::Err: std::fmt::Display,
    {
        let val = match std::env::var(key) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Config Error: Failed to load {} from environment: {}, using default value: {}", key, e, default);
                return Ok(default);
            }
        };

        match val.parse::<T>() {
            Ok(parsed) => Ok(parsed),
            Err(e) => {
                eprintln!("Config Error: Failed to parse environment variable '{}': {}, using default value: {}", key, e, default);
                Ok(default)
            }
        }
    }
}