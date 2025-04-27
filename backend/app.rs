use std::sync::Arc;

use ntex::web::{self, HttpServer};

use crate::{config::config::Config, database::DbPool};
use crate::errors::{Error, Result, ServerError};
use crate::api::auth;

pub struct AppState {
    pub config: Config,
    pub db_pool: DbPool
}

pub struct App {
    state: Arc<AppState>
}

impl App {
    pub fn new() -> Result<Self> {
        let config = Config::from_env()?;
        let db_pool = DbPool::new(&config.database_url, config.database_pool_size)?;
        
        let state = Arc::new(AppState { config, db_pool });
        
        Ok(App { state })
    }

    pub async fn run(self) -> Result<()> {
        println!("TeaPOS backend is running at http://{}:{}", 
                 self.state.config.server_address, 
                 self.state.config.server_port);
        
        let state = self.state.clone();
        
        let result = HttpServer::new(move || {
            let state = state.clone();
            web::App::new()
                .state(state.clone())
                .wrap(crate::middlewares::response_middleware::Response)
                .configure(auth::configure)
        })
        .bind((self.state.config.server_address.clone(), self.state.config.server_port))?
        .run()
        .await;
        
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Server(ServerError::Io(e)))
        }
    }
}