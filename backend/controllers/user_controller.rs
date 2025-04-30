use ntex::web::{self, HttpResponse};
use ntex::web::types::{Json, State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use anyhow::anyhow;

use crate::app::AppState;
use crate::error::{Result, Error};
use crate::models::user::{User, UserRole};
use crate::middlewares::auth_middleware::UserInfo;

#[derive(Deserialize, Debug)]
pub struct UserUpdateRequest {
    pub fullname: String,
    pub whatsapp: String,
}

#[derive(Deserialize, Debug)]
pub struct AdminUserUpdateRequest {
    pub fullname: String,
    pub whatsapp: String,
    pub role: String,
}

#[derive(Deserialize, Debug)]
pub struct PasswordUpdateRequest {
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub fullname: String,
    pub role: String,
}

#[derive(Serialize, Debug)]
pub struct UserDetailResponse {
    pub id: i32,
    pub username: String,
    pub fullname: String,
    pub whatsapp: String,
    pub role: String,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn get_current_user(req: web::HttpRequest, state: State<Arc<AppState>>) -> Result<HttpResponse> {
    let user_id = req.user_id().ok_or(Error::ForbiddenError)?;
    
    let mut conn = state.db_pool.get_connection().await?;
    let user = User::find_by_id(user_id, &mut conn).await?;
    
    let response = UserResponse {
        id: user.id,
        username: user.username,
        fullname: user.fullname,
        role: user.role.to_string(),
    };
    
    Ok(HttpResponse::Ok().json(&response))
}

pub async fn get_user_details(req: web::HttpRequest, state: State<Arc<AppState>>) -> Result<HttpResponse> {
    let user_id = req.user_id().ok_or(Error::ForbiddenError)?;
    
    let mut conn = state.db_pool.get_connection().await?;
    let user = User::find_by_id(user_id, &mut conn).await?;
    
    let response = UserDetailResponse {
        id: user.id,
        username: user.username,
        fullname: user.fullname,
        whatsapp: user.whatsapp,
        role: user.role.to_string(),
        created_at: user.created_at.to_string(),
        updated_at: user.updated_at.to_string(),
    };
    
    Ok(HttpResponse::Ok().json(&response))
}

pub async fn update_user(req: web::HttpRequest, state: State<Arc<AppState>>, body: Json<UserUpdateRequest>) -> Result<HttpResponse> {
    let user_id = req.user_id().ok_or(Error::ForbiddenError)?;
    
    let mut conn = state.db_pool.get_connection().await?;
    let mut user = User::find_by_id(user_id, &mut conn).await?;
    
    user.fullname = body.fullname.clone();
    user.whatsapp = body.whatsapp.clone();
    
    let updated_user = user.update(&mut conn).await?;
    
    let response = UserResponse {
        id: updated_user.id,
        username: updated_user.username,
        fullname: updated_user.fullname,
        role: updated_user.role.to_string(),
    };
    
    Ok(HttpResponse::Ok().json(&response))
}

pub async fn update_password(req: web::HttpRequest, state: State<Arc<AppState>>, body: Json<PasswordUpdateRequest>) -> Result<HttpResponse> {
    if body.new_password != body.confirm_password {
        return Err(Error::ApiError(anyhow!("New password and confirmation do not match")));
    }
    
    if body.new_password.len() < 8 {
        return Err(Error::ApiError(anyhow!("Password must be at least 8 characters long")));
    }
    
    let user_id = req.user_id().ok_or(Error::ForbiddenError)?;
    
    let mut conn = state.db_pool.get_connection().await?;
    let user = User::find_by_id(user_id, &mut conn).await?;
    
    if !User::verify_password(&user.password, &body.current_password)? {
        return Err(Error::ApiError(anyhow!("Current password is incorrect")));
    }
    
    let mut user_to_update = user;
    user_to_update.password = User::hash_password(&body.new_password)?;
    
    let _updated_user = user_to_update.update(&mut conn).await?;
    
    Ok(HttpResponse::Ok().json(&serde_json::json!({ "success": true, "message": "Password updated successfully" })))
}

// Admin-only endpoint to get any user's details
pub async fn admin_get_user(req: web::HttpRequest, state: State<Arc<AppState>>, path: web::types::Path<(i32,)>) -> Result<HttpResponse> {
    let user_role = req.user_role().ok_or(Error::ForbiddenError)?;
    
    if user_role != "admin" && user_role != "superadmin" {
        return Err(Error::ForbiddenError);
    }
    
    let user_id = path.0;
    
    let mut conn = state.db_pool.get_connection().await?;
    let user = User::find_by_id(user_id, &mut conn).await?;
    
    let response = UserDetailResponse {
        id: user.id,
        username: user.username,
        fullname: user.fullname,
        whatsapp: user.whatsapp,
        role: user.role.to_string(),
        created_at: user.created_at.to_string(),
        updated_at: user.updated_at.to_string(),
    };
    
    Ok(HttpResponse::Ok().json(&response))
}

// Admin-only endpoint to update any user
pub async fn admin_update_user(req: web::HttpRequest, state: State<Arc<AppState>>, path: web::types::Path<(i32,)>, body: Json<AdminUserUpdateRequest>) -> Result<HttpResponse> {
    let user_role = req.user_role().ok_or(Error::ForbiddenError)?;
    
    if user_role != "admin" && user_role != "superadmin" {
        return Err(Error::ForbiddenError);
    }
    
    let user_id = path.0;
    
    let mut conn = state.db_pool.get_connection().await?;
    let mut user = User::find_by_id(user_id, &mut conn).await?;
    
    user.fullname = body.fullname.clone();
    user.whatsapp = body.whatsapp.clone();
    
    // Only allow role updates if the current user is an admin/superadmin
    if let Ok(new_role) = body.role.parse::<UserRole>() {
        // Prevent changing superadmin roles unless you're a superadmin
        if user.role == UserRole::SuperAdmin && user_role != "superadmin" {
            return Err(Error::ApiError(anyhow!("Only superadmins can modify other superadmins")));
        }
        
        // Prevent setting someone to superadmin unless you're a superadmin
        if new_role == UserRole::SuperAdmin && user_role != "superadmin" {
            return Err(Error::ApiError(anyhow!("Only superadmins can create new superadmins")));
        }
        
        user.role = new_role;
    } else {
        return Err(Error::ApiError(anyhow!("Invalid role specified")));
    }
    
    let updated_user = user.update(&mut conn).await?;
    
    let response = UserResponse {
        id: updated_user.id,
        username: updated_user.username,
        fullname: updated_user.fullname,
        role: updated_user.role.to_string(),
    };
    
    Ok(HttpResponse::Ok().json(&response))
}

// Admin-only endpoint to list all users
pub async fn list_users(req: web::HttpRequest, state: State<Arc<AppState>>) -> Result<HttpResponse> {
    let user_role = req.user_role().ok_or(Error::ForbiddenError)?;
    
    if user_role != "admin" && user_role != "superadmin" {
        return Err(Error::ForbiddenError);
    }
    
    let mut conn = state.db_pool.get_connection().await?;
    let users = User::get_all(&mut conn).await?;
    
    let response: Vec<UserResponse> = users.into_iter().map(|user| {
        UserResponse {
            id: user.id,
            username: user.username,
            fullname: user.fullname,
            role: user.role.to_string(),
        }
    }).collect();
    
    Ok(HttpResponse::Ok().json(&response))
}
