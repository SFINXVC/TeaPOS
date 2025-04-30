use anyhow::anyhow;
use redis::{Client, aio::MultiplexedConnection};

use crate::config::config::Config;
use crate::error::{Error, Result};

pub struct RedisService {
    client: Client,
    connection: MultiplexedConnection,
}

impl RedisService {
    pub async fn new(config: &Config) -> Result<Self> {
        let client = Client::open(config.redis_url.as_str())
            .map_err(|e| Error::RedisError(anyhow!("Failed to connect to Redis: {}", e)))?;

        // Get a multiplexed connection which can be shared between threads
        let connection = client.get_multiplexed_tokio_connection().await
            .map_err(|e| Error::RedisError(anyhow!("Failed to get Redis connection: {}", e)))?;

        // Test the connection
        let _: () = redis::cmd("PING").query_async(&mut connection.clone()).await
            .map_err(|e| Error::RedisError(anyhow!("Failed to ping Redis: {}", e)))?;

        Ok(Self { client, connection })
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }
    
    pub fn get_connection(&self) -> MultiplexedConnection {
        self.connection.clone()
    }
}