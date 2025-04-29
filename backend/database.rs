use diesel_async::{pooled_connection::{
    deadpool::{BuildError, Pool, PoolError, Object}, 
    AsyncDieselConnectionManager
}, AsyncPgConnection};

use anyhow::anyhow;
use crate::error::{Error, Result};

pub struct DbPool {
    pool: Pool<AsyncPgConnection>
}

impl From<BuildError> for Error {
    fn from(e: BuildError) -> Self {
        Error::DatabaseError(anyhow!("Failed to build database connection pool: {}", e))
    }
}

impl From<PoolError> for Error {
    fn from(e: PoolError) -> Self {
        Error::DatabaseError(anyhow!("Failed to get database connection: {}", e))
    }
}

impl DbPool {
    pub fn new(url: &str, pool_size: u32) -> Result<DbPool> {
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);
        let pool = Pool::builder(config)
            .max_size(pool_size as usize)
            .build()?;

        Ok(DbPool { pool })
    }
    
    pub async fn get_connection(&self) -> std::result::Result<Object<AsyncPgConnection>, PoolError> {
        self.pool.get().await
    }
}