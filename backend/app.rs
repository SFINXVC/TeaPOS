use thiserror::Error;
use ntex::web::{self, HttpServer};

use crate::config::config::Config;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to load config: {0}")]
    ConfigLoadFailed(#[from] envy::Error),
    #[error("Failed to bind server: {0}")]
    ServerBindFailed(#[from] std::io::Error),
    #[error("Database connection failed: {0}")]
    DatabaseConnectionFailed(#[from] diesel::ConnectionError),
}

pub struct App {
    config: Config,
}

impl App {
    pub fn new() -> Result<Self, AppError> {
        let config = Config::from_env()?;

        Ok(App { config })
    }

    pub async fn run(&self) -> Result<(), AppError> {
        println!("TeaPOS backend is running at http://{}:{}", self.config.server_address, self.config.server_port);
        
        let result = HttpServer::new(|| {
            web::App::new().route("/", web::get().to(|| async { web::HttpResponse::Ok().body("Hello, world!") }))
        })
        .bind((self.config.server_address.clone(), self.config.server_port))?
        .run()
        .await;
        
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError::ServerBindFailed(e))
        }
    }
}
