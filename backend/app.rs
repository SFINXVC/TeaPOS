use std::sync::Arc;

use ntex::web::{self, HttpServer};

use crate::services::redis_service::RedisService;
use crate::services::session_service::SessionService;
use crate::services::token_service::TokenService;
use crate::{config::config::Config, database::DbPool};
use crate::errors::{Error, Result, ServerError};
use crate::api::auth;
use crate::seeds;

async fn not_found() -> Result<web::HttpResponse> {
    Err(Error::Server(ServerError::Forbidden))
}

pub struct AppState {
    pub config: Config,
    pub db_pool: DbPool,

    // services are registered here (to avoid load them multiple times)
    pub token_service: TokenService,
    pub redis_service: Arc<RedisService>,
    pub session_service: SessionService,
}

pub struct App {
    state: Arc<AppState>
}

impl App {
    pub fn new() -> Result<Self> {
        let config = Config::from_env()?;
        let db_pool = DbPool::new(&config.database_url, config.database_pool_size)?;

        let redis_service = Arc::new(RedisService::new(&config)?);

        let token_service = TokenService::new(&config);

        let session_service = SessionService::new(redis_service.clone(), &config);

        let state = Arc::new(AppState { config, db_pool, token_service, redis_service, session_service });

        Ok(App { state })
    }

    pub async fn run_seeds(&self, specific_seeder: Option<&str>) -> Result<()> {
        println!("Running database seeds...");
        
        let mut conn = self.state.db_pool.get_connection().await?;
        
        match specific_seeder {
            Some(seeder_name) => {
                println!("Running specific seeder: {}", seeder_name);
                seeds::run_seeder(seeder_name, &mut conn).await?
            },
            None => {
                println!("Running all seeders");
                seeds::run_all_seeds(&mut conn).await?
            }
        }
        
        Ok(())
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
                .wrap(crate::middlewares::auth_middleware::Auth)
                .wrap(crate::middlewares::response_middleware::Response)
                .configure(auth::configure)
                .default_service(web::to(not_found))
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