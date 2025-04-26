use diesel_async::{pooled_connection::{
    deadpool::{BuildError, Pool, PoolError, Object}, 
    AsyncDieselConnectionManager
}, AsyncPgConnection};

use crate::errors::{Error, DatabaseError, Result};

pub struct DbPool {
    pool: Pool<AsyncPgConnection>
}

impl From<BuildError> for Error {
    fn from(e: BuildError) -> Self {
        Error::Database(DatabaseError::Connection(e.to_string()))
    }
}

impl From<PoolError> for Error {
    fn from(e: PoolError) -> Self {
        Error::Database(DatabaseError::Pool(e.to_string()))
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