use std::sync::Arc;
use anyhow::anyhow;
use thiserror::Error;
use chrono::{DateTime, Duration, Utc};
use redis::{Commands, RedisResult};
use serde::{Serialize, Deserialize};

use crate::config::config::Config;
use crate::error::{Error as AppError, Result};
use crate::services::redis_service::RedisService;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceInfo {
    pub user_agent: String,
    pub ip_address: String,
    pub device_id: String,
    pub last_active: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionData {
    pub user_id: i32,
    pub device_info: DeviceInfo,
    pub is_valid: bool,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>
}

pub struct SessionService {
    redis_service: Arc<RedisService>,
    session_expiry: u64, // in sec    
}



#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Invalid session token")]
    InvalidSessionToken
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
            session_expiry: config.ref_token_expiry as u64
        }
    }

    async fn is_same_device(&self, stored: &DeviceInfo, current: &DeviceInfo) -> bool {
        stored.device_id == current.device_id && 
        stored.ip_address == current.ip_address && 
        stored.user_agent == current.user_agent
    }

    pub async fn create_session(&self, user_id: i32, device_info: DeviceInfo, token_id: &str) -> Result<()> {
        let mut conn = self.redis_service.get_connection()?;

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
            .map_err(|_| AppError::ServiceError(anyhow!("Failed to serialize session data")))?;

        let _: () = redis::pipe()
            .atomic()
            .set_ex(&session_key, session_json, self.session_expiry)
            .sadd(&user_sessions_key, token_id)
            .expire(&user_sessions_key, self.session_expiry as i64)
            .query(&mut conn)
            .map_err(|e| AppError::ServiceError(anyhow!("Redis command error: {}", e)))?;
        Ok(())
    }

    pub async fn validate_session(&self, token_id: &str, current_device_info: DeviceInfo) -> Result<SessionData> {
        let mut conn = self.redis_service.get_connection()?;
        
        let session_key = format!("session:{}", token_id);
        
        let session_json: String = conn.get(&session_key)
            .map_err(|_| SessionError::InvalidSessionToken)?;
        
        let mut session_data: SessionData = serde_json::from_str(&session_json)
            .map_err(|_| AppError::ServiceError(anyhow!("Failed to deserialize session data")))?;
        
        if !session_data.is_valid {
            return Err(SessionError::InvalidSessionToken.into());
        }

        if !self.is_same_device(&session_data.device_info, &current_device_info).await {
            return Err(SessionError::InvalidSessionToken.into());
        }

        // update last active time to keep track of user activity
        session_data.device_info.last_active = Utc::now();

        let updated_json = serde_json::to_string(&session_data)
            .map_err(|_| AppError::ServiceError(anyhow!("Failed to serialize session data")))?;

        let _: () = conn.set_ex(&session_key, updated_json, self.session_expiry)
            .map_err(|e| AppError::ServiceError(anyhow!("Redis command error: {}", e)))?;

        Ok(session_data)
    }

    pub async fn invalidate_session(&self, token_id: &str) -> Result<()> {
        let mut conn = self.redis_service.get_connection()?;
        
        let session_key = format!("session:{}", token_id);
        
        let session_json: RedisResult<String> = conn.get(&session_key);

        if let Ok(json) = session_json {
            if let Ok(mut session_data) = serde_json::from_str::<SessionData>(&json) {
                // mark the session as invalid, but let's keep it for a while for audit stuffs
                session_data.is_valid = false;

                let updated_json = serde_json::to_string(&session_data)
                    .map_err(|_| AppError::ServiceError(anyhow!("Failed to serialize session data")))?;

                let _: () = conn.set_ex(&session_key, updated_json, 86400) // keep it for 24h
                    .map_err(|e| AppError::ServiceError(anyhow!("Redis error: {}", e)))?;

                // remove from user's active sessions
                let user_sessions_key = format!("user:{}:sessions", session_data.user_id);
                let _: () = conn.srem(&user_sessions_key, token_id)
                    .map_err(|e| AppError::ServiceError(anyhow!("Redis error: {}", e)))?;
            }
        }
        
        Ok(())
    }

    pub async fn get_user_sessions(&self, user_id: i32) -> Result<Vec<SessionData>> {
        let mut conn = self.redis_service.get_connection()?;

        let user_sessions_key = format!("user:{}:sessions", user_id);

        let session_ids: Vec<String> = conn.smembers(&user_sessions_key)
            .map_err(|e| AppError::ServiceError(anyhow!("Redis command error: {}", e)))?;

        let mut sessions = Vec::new();
        
        for token_id in session_ids {
            let session_key = format!("session:{}", token_id);
            
            if let Ok(session_json) = conn.get::<_, String>(&session_key) {
                if let Ok(session_data) = serde_json::from_str::<SessionData>(&session_json) {
                    if session_data.is_valid {
                        sessions.push(session_data);
                    }
                }
            }
        }

        Ok(sessions)
    }

    pub async fn logout_all_sessions(&self, user_id: i32) -> Result<()> {
        let mut conn = self.redis_service.get_connection()?;

        let user_sessions_key = format!("user:{}:sessions", user_id);
        
        let session_ids: Vec<String> = conn.smembers(&user_sessions_key)
            .map_err(|e| AppError::ServiceError(anyhow!("Redis command error: {}", e)))?;

        for token_id in session_ids {
            self.invalidate_session(&token_id).await?;
        }

        Ok(())
    }

    pub async fn update_session_activity(&self, token_id: &str, device_info: DeviceInfo) -> Result<()> {
        let mut conn = self.redis_service.get_connection()?;
        
        let session_key = format!("session:{}", token_id);
        
        let session_json: String = conn.get(&session_key)
            .map_err(|_| SessionError::InvalidSessionToken)?;
        
        let mut session_data: SessionData = serde_json::from_str(&session_json)
            .map_err(|_| AppError::ServiceError(anyhow!("Failed to deserialize session data")))?;
        
        if !session_data.is_valid {
            return Err(SessionError::InvalidSessionToken.into());
        }

        // Update device info and last active time
        session_data.device_info = device_info;

        let updated_json = serde_json::to_string(&session_data)
            .map_err(|_| AppError::ServiceError(anyhow!("Failed to serialize session data")))?;

        let _: () = conn.set_ex(&session_key, updated_json, self.session_expiry)
            .map_err(|e| AppError::ServiceError(anyhow!("Redis command error: {}", e)))?;

        Ok(())
    }
}