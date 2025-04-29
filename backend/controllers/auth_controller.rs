use chrono::Utc;
use ntex::web::{self, HttpResponse};
use ntex::web::types::{Json, State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Arc;
use anyhow::anyhow;

use crate::app::AppState;
use crate::error::{Result, Error};
use crate::models::user::{NewUser, User, UserRole};
use crate::services::session_service::DeviceInfo;

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
    pub device_id: String,
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
    pub password_confirm: String,
    pub whatsapp: String
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
    let mut conn = state.db_pool.get_connection().await?;

    let user = match User::find_by_username(&req.username, &mut conn).await {
        Ok(user) => user,
        Err(_) => return Err(Error::ApiError(anyhow!("Invalid credentials")))
    };

    if !User::verify_password(&user.password, &req.password)? {
        return Err(Error::ApiError(anyhow!("Invalid credentials")));
    }
    
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
    
    let current_device_info = DeviceInfo {
        user_agent: http_req.headers().get("User-Agent")
            .map(|v| v.to_str().unwrap_or("Unknown").to_string())
            .unwrap_or_else(|| "Unknown".to_string()),
        ip_address: http_req.peer_addr()
            .map(|addr| addr.ip().to_string())
            .unwrap_or("Unknown".to_string()),
        device_id: req.device_id.clone(),
        last_active: Utc::now()
    };
    
    let session = match state.session_service.validate_session(&refresh_claims.jti, current_device_info).await {
        Ok(session) => session,
        Err(e) => { 
            return Err(Error::ApiError(anyhow!("Failed to invalidate session: {}", e)))
        }
    };

    let mut conn = state.db_pool.get_connection().await?;
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
    let mut conn = state.db_pool.get_connection().await?;
    
    if req.password != req.password_confirm {
        return Err(Error::ApiError(anyhow!("'password_confirm' doesn't match with 'password'")));
    }

    let new_user = NewUser {
        username: req.username.clone(),
        fullname: req.fullname.clone(),
        password: req.password.clone(),
        whatsapp: req.whatsapp.clone(),
        role: UserRole::User
    };
    
    let user = User::create_and_return(new_user, &mut conn).await?;
    
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