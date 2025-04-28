use dotenv::dotenv;
use serde::Deserialize;

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
    pub fn from_env() -> Result<Self, crate::errors::Error> {
        dotenv().ok();
        
        let default_config = Self::default();
        
        // Try to get values from environment, use defaults if not found
        let server_address = Self::get_env_or_default("SERVER_ADDRESS", &default_config.server_address);
        let server_port = Self::get_env_or_default_parse("SERVER_PORT", default_config.server_port);
        let database_url = Self::get_env_or_default("DATABASE_URL", &default_config.database_url);
        let database_pool_size = Self::get_env_or_default_parse("DATABASE_POOL_SIZE", default_config.database_pool_size);
        let redis_url = Self::get_env_or_default("REDIS_URL", &default_config.redis_url);
        let acc_token_secret = Self::get_env_or_default("ACC_TOKEN_SECRET", &default_config.acc_token_secret);
        let ref_token_secret = Self::get_env_or_default("REF_TOKEN_SECRET", &default_config.ref_token_secret);
        let acc_token_expiry = Self::get_env_or_default_parse("ACC_TOKEN_EXPIRY", default_config.acc_token_expiry);
        let ref_token_expiry = Self::get_env_or_default_parse("REF_TOKEN_EXPIRY", default_config.ref_token_expiry);
        
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
    
    fn get_env_or_default(key: &str, default: &str) -> String {
        match std::env::var(key) {
            Ok(value) => value,
            Err(_) => {
                println!("Warning: Environment variable '{}' not found, using default value", key);
                default.to_string()
            }
        }
    }
    
    fn get_env_or_default_parse<T: std::str::FromStr + std::fmt::Display>(key: &str, default: T) -> T {
        match std::env::var(key) {
            Ok(value) => {
                match value.parse::<T>() {
                    Ok(parsed) => parsed,
                    Err(_) => {
                        println!("Warning: Could not parse environment variable '{}', using default value: {}", key, default);
                        default
                    }
                }
            },
            Err(_) => {
                println!("Warning: Environment variable '{}' not found, using default value: {}", key, default);
                default
            }
        }
    }
}