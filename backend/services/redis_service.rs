use anyhow::anyhow;
use redis::{Client, Connection};

use crate::config::config::Config;
use crate::error::{Error, Result};

pub struct RedisService {
    client: Client
}

impl RedisService {
    pub fn new(config: &Config) -> Result<Self> {
        let client = Client::open(config.redis_url.as_str())
            .map_err(|e| { Error::RedisError(anyhow!("Failed to connect to Redis: {}", e)) })?;

        // test the connection
        let mut conn = client.get_connection()
            .map_err(|e| { Error::RedisError(anyhow!("Failed to get Redis connection: {}", e)) })?;

        let _: () = redis::cmd("PING").query(&mut conn)
            .map_err(|e| { Error::RedisError(anyhow!("Failed to ping Redis: {}", e)) })?;

        Ok(Self { client })
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }

    pub fn get_connection(&self) -> Result<Connection> {
        self.client
            .get_connection()
            .map_err(|e| { Error::RedisError(anyhow!("Failed to get Redis connection: {}", e)) })
    }
}