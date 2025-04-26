use ntex::web::HttpResponse;
use ntex::web::types::{Json, State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::app::AppState;
use crate::errors::{Result, Error, AuthError, DatabaseError, SuccessResponse};
use crate::models::user::NewUser;
use crate::services::user_service;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub fullname: String,
    pub password: String,
    pub whatsapp: String,
    pub role: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub fullname: String,
    pub whatsapp: String,
    pub role: String,
}

pub async fn login(state: State<Arc<AppState>>, req: Json<LoginRequest>) -> Result<HttpResponse> {
    let mut conn = state.db_pool.get_connection().await
        .map_err(|e| Error::Database(DatabaseError::Pool(e.to_string())))?;

    let user = user_service::login_user(&mut conn, &req.username, &req.password).await?;
    
    let response = UserResponse {
        id: user.id,
        username: user.username,
        fullname: user.fullname,
        whatsapp: user.whatsapp,
        role: user.role.to_string(),
    };

    Ok(HttpResponse::Ok().json(&SuccessResponse::new(response)))
}

pub async fn register(state: State<Arc<AppState>>, req: Json<RegisterRequest>) -> Result<HttpResponse> {
    let mut conn = state.db_pool.get_connection().await
        .map_err(|e| Error::Database(DatabaseError::Pool(e.to_string())))?;
    
    let role = req.role.parse()
        .map_err(|_| Error::Auth(AuthError::InvalidCredentials))?;
    
    let new_user = NewUser {
        username: req.username.clone(),
        fullname: req.fullname.clone(),
        password: req.password.clone(),
        whatsapp: req.whatsapp.clone(),
        role,
    };
    
    let user = user_service::register_user(&mut conn, new_user).await?;
    
    let response = UserResponse {
        id: user.id,
        username: user.username,
        fullname: user.fullname,
        whatsapp: user.whatsapp,
        role: user.role.to_string(),
    };

    Ok(HttpResponse::Created().json(&SuccessResponse::new(response)))
}