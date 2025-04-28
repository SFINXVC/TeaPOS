use diesel::result::Error as DieselError;
use thiserror::Error;
use ntex::web;
use ntex::http::StatusCode;
use envy;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("{0}")]
    Database(#[from] DatabaseError),

    #[error("{0}")]
    Auth(#[from] AuthError),

    #[error("{0}")]
    Validation(#[from] ValidationError),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("{0}")]
    Server(#[from] ServerError),

    #[error("{0}")]
    Redis(#[from] RedisError),

    #[error("Configuration error: {0}")]
    Config(String),
}

impl web::error::WebResponseError for Error {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        let (status_code, message) = match self {
            Error::Auth(auth_err) => {
                // Log the actual error for debugging but return a generic message
                log_error(format!("Authentication error: {}", auth_err));
                (StatusCode::UNAUTHORIZED, "Authentication failed".to_string())
            },
            Error::Validation(val_err) => {
                (StatusCode::BAD_REQUEST, val_err.to_string())
            },
            Error::NotFound(_) => {
                (StatusCode::NOT_FOUND, "An internal server error occurred".to_string())
            },
            Error::Database(db_err) => {
                log_error(db_err);
                (StatusCode::INTERNAL_SERVER_ERROR, "An internal server error occurred".to_string())
            },
            Error::Server(srv_err) => {
                if *srv_err != ServerError::Forbidden {
                    log_error(srv_err);
                }
                (StatusCode::INTERNAL_SERVER_ERROR, "An internal server error occurred".to_string())
            },
            Error::Redis(redis_err) => {
                log_error(redis_err);
                (StatusCode::INTERNAL_SERVER_ERROR, "An internal server error occurred".to_string())
            },
            Error::Config(err_msg) => {
                log_error(err_msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "An internal server error occurred".to_string())
            },
        };
        
        web::HttpResponse::build(status_code).body(message)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum DatabaseError {
    #[error("{0}")]
    Query(#[from] DieselError),
    
    #[error("Database connection failed: {0}")]
    Connection(String),
    
    #[error("Database pool error: {0}")]
    Pool(String),
}

#[derive(Error, Debug, PartialEq)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("Token expired")]
    TokenExpired,
    
    #[error("Invalid token")]
    InvalidToken,
    
    #[error("Unauthorized")]
    Unauthorized,
}

#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Missing field: {0}")]
    MissingField(String),
    
    #[error("Resource already exists: {0}")]
    AlreadyExists(String),
}

#[derive(Error, Debug, PartialEq)]
pub enum RedisError {
    #[error("Redis connection failed: {0}")]
    Connection(redis::RedisError),
    
    #[error("Redis command failed: {0}")]
    Command(redis::RedisError)
}

impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        Error::Redis(RedisError::Connection(err))
    }
}

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unexpected error: {0}")]
    Unexpected(String),

    #[error("You're not allowed to access this resource")]
    Forbidden
}

// lol, this is ugly asf. But nvm. 
impl PartialEq for ServerError {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other),
            (ServerError::Forbidden, ServerError::Forbidden)
        )
    }
}

pub type Result<T> = std::result::Result<T, Error>;

fn log_error<T: std::fmt::Display>(details: T) {
    eprintln!("[ERROR] {}", details);
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Server(ServerError::Io(err))
    }
}

impl From<envy::Error> for Error {
    fn from(err: envy::Error) -> Self {
        Error::Config(err.to_string())
    }
}
