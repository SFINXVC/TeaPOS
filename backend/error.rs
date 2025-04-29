use ntex::web;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database Error: {0}")]
    DatabaseError(anyhow::Error),
    
    #[error("Service Error: {0}")]
    ServiceError(anyhow::Error),
    
    #[error("Controller Error: {0}")]
    ControllerError(anyhow::Error),
    
    #[error("{0}")]
    ApiError(anyhow::Error),
    
    #[error("Config Error: {0}")]
    ConfigError(anyhow::Error),
    
    #[error("Redis Error: {0}")]
    RedisError(anyhow::Error),
    
    #[error("Io Error: {0}")]
    IoError(anyhow::Error),

    #[error("Forbidden Error")]
    ForbiddenError,
    
    #[error(transparent)]
    GeneralError(anyhow::Error),
}

fn log_error(error: &Error) {
    eprintln!("[ERROR] {}", error);
}

const FORBIDDEN_MESSAGE: &str = "You don't have permission to access this resource.";
const INTERNAL_ERROR_MESSAGE: &str = "An internal server error occurred while trying to process your request.";

impl web::error::WebResponseError for Error {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        match self {            
            Error::DatabaseError(_) => {
                log_error(self);
                web::HttpResponse::InternalServerError()
                    .content_type("text/plain")
                    .body(INTERNAL_ERROR_MESSAGE)
            },
            
            Error::ServiceError(_) => {
                log_error(self);
                web::HttpResponse::InternalServerError()
                    .content_type("text/plain")
                    .body(INTERNAL_ERROR_MESSAGE)
            },
            
            Error::ControllerError(_) | Error::ApiError(_) => {
                // log_error(self); -- no need to log this
                web::HttpResponse::BadRequest()
                    .content_type("text/plain")
                    .body(self.to_string())
            },
            
            Error::ConfigError(_) => {
                log_error(self);
                web::HttpResponse::InternalServerError()
                    .content_type("text/plain")
                    .body(INTERNAL_ERROR_MESSAGE)
            },
            
            Error::RedisError(_) => {
                log_error(self);
                web::HttpResponse::InternalServerError()
                    .content_type("text/plain")
                    .body(INTERNAL_ERROR_MESSAGE)
            },
            
            Error::IoError(_) => {
                log_error(self);
                web::HttpResponse::InternalServerError()
                    .content_type("text/plain")
                    .body(INTERNAL_ERROR_MESSAGE)
            },
            
            Error::ForbiddenError => {
                log_error(self);
                web::HttpResponse::Forbidden()
                    .content_type("text/plain")
                    .body(FORBIDDEN_MESSAGE)
            },
            
            Error::GeneralError(_) => { 
                log_error(self);
                web::HttpResponse::InternalServerError()
                    .content_type("text/plain")
                    .body(INTERNAL_ERROR_MESSAGE)
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;