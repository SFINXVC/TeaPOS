use std::sync::Arc;
use thiserror::Error;
use chrono::{DateTime, Duration, Utc};
use redis::{RedisResult, AsyncCommands};
use serde::{Serialize, Deserialize};

use crate::config::config::Config;
use crate::error::{Error as AppError, Result};
use crate::services::redis_service::RedisService;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DeviceInfo {
    pub user_agent: String,
    pub ip_address: String,
    pub device_id: String,
    pub last_active: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionData {
    pub user_id: i64,
    pub device_info: DeviceInfo,
    pub is_valid: bool,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>
}

pub struct SessionService {
    redis_service: Arc<RedisService>,
    session_expiry: u64, // in sec
    invalid_session_retention: u64, // in sec
}

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Invalid session token")]
    InvalidSessionToken,
    #[error("Session expired")]
    SessionExpired,
    #[error("Device mismatch")]
    DeviceMismatch,
    #[error("Redis error: {0}")]
    RedisError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String)
}

impl From<SessionError> for AppError {
    fn from(error: SessionError) -> Self {
        AppError::ServiceError(error.into())
    }
}

impl SessionService {
    pub fn new(redis_service: Arc<RedisService>, config: &Config) -> Self {
        Self {
            redis_service,
            session_expiry: config.ref_token_expiry as u64,
            invalid_session_retention: 86400
        }
    }

    async fn is_same_device(&self, stored: &DeviceInfo, current: &DeviceInfo) -> bool {
        stored.device_id == current.device_id && 
        stored.user_agent == current.user_agent
    }

    pub async fn create_session(&self, user_id: i64, device_info: DeviceInfo, token_id: &str) -> Result<()> {
        let mut conn = self.redis_service.get_connection();

        let now = Utc::now();
        let exp = now + Duration::seconds(self.session_expiry as i64);

        let session_data = SessionData {
            user_id,
            device_info,
            is_valid: true,
            created_at: now,
            expires_at: exp,
        };
        
        let session_key = format!("session:{}", token_id);
        let user_sessions_key = format!("user:{}:sessions", user_id);

        let session_json = serde_json::to_string(&session_data)
            .map_err(|e| SessionError::SerializationError(e.to_string()))?;

        let mut pipe = redis::pipe();
        pipe.atomic()
            .set_ex(&session_key, session_json, self.session_expiry)
            .sadd(&user_sessions_key, token_id)
            .expire(&user_sessions_key, self.session_expiry as i64);
        
        let _: () = pipe.query_async(&mut conn).await
            .map_err(|e| SessionError::RedisError(e.to_string()))?;
        
        Ok(())
    }

    pub async fn validate_session(&self, token_id: &str, current_device_info: DeviceInfo) -> Result<SessionData> {
        let mut conn = self.redis_service.get_connection();
        
        let session_key = format!("session:{}", token_id);
        
        let session_json: String = conn.get(&session_key).await
            .map_err(|_| SessionError::InvalidSessionToken)?;
        
        let mut session_data: SessionData = serde_json::from_str(&session_json)
            .map_err(|e| SessionError::SerializationError(e.to_string()))?;
        
        if !session_data.is_valid {
            return Err(SessionError::InvalidSessionToken.into());
        }
        
        let now = Utc::now();
        if session_data.expires_at < now {
            return Err(SessionError::SessionExpired.into());
        }

        if !self.is_same_device(&session_data.device_info, &current_device_info).await {
            return Err(SessionError::DeviceMismatch.into());
        }

        session_data.device_info.last_active = now;
        
        let updated_json = serde_json::to_string(&session_data)
            .map_err(|e| SessionError::SerializationError(e.to_string()))?;

        let mut pipe = redis::pipe();
        pipe.atomic()
            .set_ex(&session_key, updated_json, self.session_expiry);
            
        let _: () = pipe.query_async(&mut conn).await
            .map_err(|e| SessionError::RedisError(e.to_string()))?;

        Ok(session_data)
    }

    pub async fn invalidate_session(&self, token_id: &str) -> Result<()> {
        let mut conn = self.redis_service.get_connection();
        
        let session_key = format!("session:{}", token_id);
        
        let session_json: RedisResult<String> = conn.get(&session_key).await;

        if let Ok(json) = session_json {
            match serde_json::from_str::<SessionData>(&json) {
                Ok(mut session_data) => {
                    session_data.is_valid = false;

                    let updated_json = serde_json::to_string(&session_data)
                        .map_err(|e| SessionError::SerializationError(e.to_string()))?;

                    let mut pipe = redis::pipe();
                    pipe.atomic();
                    
                    // keep invalidated session for audit purposes but with different expiry
                    pipe.set_ex(&session_key, updated_json, self.invalid_session_retention);
                    
                    let user_sessions_key = format!("user:{}:sessions", session_data.user_id);
                    pipe.srem(&user_sessions_key, token_id);
                    
                    let _: () = pipe.query_async(&mut conn).await
                        .map_err(|e| SessionError::RedisError(e.to_string()))?;
                },
                Err(e) => {
                    // if we can't deserialize, just delete the session
                    let _: RedisResult<()> = conn.del(&session_key).await;
                    return Err(SessionError::SerializationError(e.to_string()).into());
                }
            }
        }
        
        Ok(())
    }

    pub async fn get_user_sessions(&self, user_id: i64) -> Result<Vec<SessionData>> {
        let mut conn = self.redis_service.get_connection();
        let user_sessions_key = format!("user:{}:sessions", user_id);
        let now = Utc::now();

        let session_ids: Vec<String> = conn.smembers(&user_sessions_key).await
            .map_err(|e| SessionError::RedisError(e.to_string()))?;

        let mut sessions = Vec::new();
        let mut invalid_sessions = Vec::new();
        
        for token_id in session_ids {
            let session_key = format!("session:{}", token_id);
            let json_result = conn.get::<_, String>(&session_key).await;
            
            if json_result.is_err() {
                invalid_sessions.push(token_id);
                continue;
            }
            
            let parse_result = serde_json::from_str::<SessionData>(&json_result.unwrap());
            
            if parse_result.is_err() {
                invalid_sessions.push(token_id);
                continue;
            }
            
            let session_data = parse_result.unwrap();
            
            if !session_data.is_valid || session_data.expires_at <= now {
                invalid_sessions.push(token_id);
                continue;
            }
            
            sessions.push(session_data);
        }

        if !invalid_sessions.is_empty() {
            let _: RedisResult<()> = conn.srem(&user_sessions_key, invalid_sessions.as_slice()).await;
        }

        Ok(sessions)
    }

    pub async fn logout_all_sessions(&self, user_id: i64) -> Result<()> {
        let mut conn = self.redis_service.get_connection();

        let user_sessions_key = format!("user:{}:sessions", user_id);
        
        let session_ids: Vec<String> = conn.smembers(&user_sessions_key).await
            .map_err(|e| SessionError::RedisError(e.to_string()))?;

        let mut pipe = redis::pipe();
        pipe.atomic();
        let mut session_keys = Vec::with_capacity(session_ids.len());
        
        for token_id in &session_ids {
            let session_key = format!("session:{}", token_id);
            session_keys.push(session_key.clone());
            
            match conn.get::<_, String>(&session_key).await {
                Ok(session_json) => {
                    if let Ok(mut session_data) = serde_json::from_str::<SessionData>(&session_json) {
                        session_data.is_valid = false;
                        if let Ok(updated_json) = serde_json::to_string(&session_data) {
                            pipe.set_ex(&session_key, updated_json, self.invalid_session_retention);
                        }
                    }
                },
                Err(_) => continue
            }
        }

        let _: RedisResult<()> = pipe.query_async(&mut conn).await;
        
        let _: RedisResult<()> = conn.del(&user_sessions_key).await;
        
        Ok(())
    }

    pub async fn update_session_activity(&self, token_id: &str, current_device_info: DeviceInfo) -> Result<()> {
        let mut conn = self.redis_service.get_connection();
        
        let session_key = format!("session:{}", token_id);
        
        let session_json: String = conn.get(&session_key).await
            .map_err(|_| SessionError::InvalidSessionToken)?;
        
        let mut session_data: SessionData = serde_json::from_str(&session_json)
            .map_err(|e| SessionError::SerializationError(e.to_string()))?;
        
        if !session_data.is_valid {
            return Err(SessionError::InvalidSessionToken.into());
        }
        
        let now = Utc::now();
        if session_data.expires_at < now {
            return Err(SessionError::SessionExpired.into());
        }

        if !self.is_same_device(&session_data.device_info, &current_device_info).await {
            return Err(SessionError::DeviceMismatch.into());
        }
        
        session_data.device_info.last_active = now;
        session_data.device_info.ip_address = current_device_info.ip_address;

        let updated_json = serde_json::to_string(&session_data)
            .map_err(|e| SessionError::SerializationError(e.to_string()))?;

        let mut pipe = redis::pipe();
        pipe.atomic()
            .set_ex(&session_key, updated_json, self.session_expiry);
            
        let _: () = pipe.query_async(&mut conn).await
            .map_err(|e| SessionError::RedisError(e.to_string()))?;

        Ok(())
    }
}