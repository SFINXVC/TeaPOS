use diesel::result::Error as DieselError;
use thiserror::Error;
use ntex::web;
use ntex::http::StatusCode;
use envy;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),

    #[error("Authentication error: {0}")]
    Auth(#[from] AuthError),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Server error: {0}")]
    Server(#[from] ServerError),

    #[error("Config error: {0}")]
    Config(String),
}

impl web::error::WebResponseError for Error {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        let status_code = match self {
            Error::Auth(_) => StatusCode::UNAUTHORIZED,
            Error::Validation(_) => StatusCode::BAD_REQUEST,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::Database(_) | Error::Server(_) | Error::Config(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        
        let message = match self {
            Error::Database(_) | Error::Server(_) | Error::Config(_) => 
                "Internal server error".to_string(),
            _ => self.to_string(),
        };
        
        web::HttpResponse::build(status_code)
            .json(&ErrorResponse::new(message))
    }
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Query error: {0}")]
    Query(#[from] DieselError),
    
    #[error("Connection error: {0}")]
    Connection(String),
    
    #[error("Pool error: {0}")]
    Pool(String),
}

#[derive(Error, Debug)]
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

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Missing field: {0}")]
    MissingField(String),
    
    #[error("Resource already exists: {0}")]
    AlreadyExists(String),
}

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
}

impl ErrorResponse {
    pub fn new<T: Into<String>>(message: T) -> Self {
        Self {
            success: false,
            message: message.into(),
        }
    }
}

#[derive(serde::Serialize)]
pub struct SuccessResponse<T: serde::Serialize> {
    pub success: bool,
    pub data: T,
}

impl<T: serde::Serialize> SuccessResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            success: true,
            data,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

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
