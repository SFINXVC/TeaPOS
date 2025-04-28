use chrono::Utc;
use ntex::web::{self, HttpResponse};
use ntex::web::types::{Json, State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Arc;

use crate::app::AppState;
use crate::errors::{Result, Error, AuthError, DatabaseError};
use crate::models::user::{NewUser, User};
use crate::services::session_service::DeviceInfo;
use crate::services::user_service;

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Deserialize, Debug)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub fullname: String,
    pub password: String,
    pub whatsapp: String,
    pub role: String,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub fullname: String,
    pub whatsapp: String,
    pub role: String,
}

#[derive(Serialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserResponse
}

#[derive(Serialize, Debug)]
pub struct AccessTokenResponse {
    pub user: UserResponse
}

#[derive(Serialize, Debug)]
pub struct SessionResponse {
    pub device_info: DeviceInfo,
    pub created_at: String,
    pub last_active: String
}

pub async fn login(state: State<Arc<AppState>>, req: Json<LoginRequest>, http_req: web::HttpRequest) -> Result<HttpResponse> {
    let mut conn = state.db_pool.get_connection().await
        .map_err(|e| Error::Database(DatabaseError::Pool(e.to_string())))?;

    let user = user_service::login_user(&mut conn, &req.username, &req.password).await?;
    
    let (access_token, refresh_token) = state.token_service.generate_tokens(&user)?;

    let refresh_claims = state.token_service.verify_refresh_token(&refresh_token)?;

    let device_info = DeviceInfo { 
        user_agent: http_req.headers().get("User-Agent")
            .map(|v| v.to_str().unwrap_or("Unknown").to_string())
            .unwrap_or_else(|| "Unknown".to_string()),
        ip_address: http_req.peer_addr()
            .map(|addr| addr.ip().to_string())
            .unwrap_or("Unknown".to_string()),
        device_id: Uuid::new_v4().to_string(),
        last_active: Utc::now()
    };

    state.session_service.create_session(user.id, device_info, &refresh_claims.jti).await?;

    let response = UserResponse {
        id: user.id,
        username: user.username,
        fullname: user.fullname,
        whatsapp: user.whatsapp,
        role: user.role.to_string(),
    };

    Ok(HttpResponse::Ok()
        .set_header("X-Access-Token", access_token)
        .set_header("X-Refresh-Token", refresh_token)
        .json(&response))
}

pub async fn refresh_token(state: State<Arc<AppState>>, req: Json<RefreshTokenRequest>, http_req: web::HttpRequest) -> Result<HttpResponse> {
    let refresh_claims = state.token_service.verify_refresh_token(&req.refresh_token)?;
    let session = state.session_service.validate_session(&refresh_claims.jti).await?;

    let mut conn = state.db_pool.get_connection().await
        .map_err(|e| Error::Database(DatabaseError::Pool(e.to_string())))?;

    let user = User::find_by_id(session.user_id, &mut conn).await?;

    let access_token = state.token_service.generate_access_token(&user)?;

    let device_info = DeviceInfo {
        user_agent: http_req.headers().get("User-Agent")
            .map(|v| v.to_str().unwrap_or("Unknown").to_string())
            .unwrap_or_else(|| "Unknown".to_string()),
        ip_address: http_req.peer_addr()
            .map(|addr| addr.ip().to_string())
            .unwrap_or("Unknown".to_string()),
        device_id: session.device_info.device_id,
        last_active: Utc::now()
    };

    state.session_service.update_session_activity(&refresh_claims.jti, device_info).await?;

    let response = AccessTokenResponse {
        user: UserResponse {
            id: user.id,
            username: user.username,
            fullname: user.fullname,
            whatsapp: user.whatsapp,
            role: user.role.to_string(),
        }
    };

    Ok(HttpResponse::Ok()
        .set_header("X-Access-Token", access_token)
        .json(&response))
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

    Ok(HttpResponse::Ok().json(&response))
}

pub async fn logout(state: State<Arc<AppState>>, req: Json<LogoutRequest>) -> Result<HttpResponse> {
    let refresh_claims = state.token_service.verify_refresh_token(&req.refresh_token)?;
    
    state.session_service.invalidate_session(&refresh_claims.jti).await?;
    
    Ok(HttpResponse::Ok().finish())
}