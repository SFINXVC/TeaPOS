use diesel_async::{pooled_connection::{
    deadpool::{BuildError, Pool, PoolError, Object}, 
    AsyncDieselConnectionManager
}, AsyncPgConnection};

pub struct DbPool {
    pool: Pool<AsyncPgConnection>
}

impl DbPool {
    pub fn new(url: &'static str, pool_size: u32) -> Result<DbPool, BuildError> {
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);
        let pool = Pool::builder(config)
            .max_size(pool_size as usize)
            .build()?;

        Ok(DbPool { pool })
    }
    
    pub async fn get_connection(&self) -> Result<Object<AsyncPgConnection>, PoolError> {
        self.pool.get().await
    }
}