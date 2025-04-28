use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{config::config::Config, errors::{AuthError, Error, Result}, models::user::User};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: i32,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
    pub jti: String,
    pub token_type: String // "acc" / "ref"
}

pub struct TokenService {
    access_secret: String,
    refresh_secret: String,
    access_expiry: i64, // in sec
    refresh_expiry: i64 // in sec
}

impl TokenService {
    pub fn new(config: &Config) -> Self {
        Self {
            access_secret: config.acc_token_secret.clone(),
            refresh_secret: config.ref_token_secret.clone(),
            access_expiry: config.acc_token_expiry,
            refresh_expiry: config.ref_token_expiry
        }
    }

    pub fn generate_tokens(&self, user: &User) -> Result<(String, String)> {
        let now = Utc::now();

        let acc_claims = TokenClaims {
            sub: user.id,
            role: user.role.to_string(),
            exp: (now + Duration::seconds(self.access_expiry)).timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
            token_type: "acc".to_string()
        };

        let acc_token = encode(
            &Header::new(Algorithm::HS256),
            &acc_claims,
            &EncodingKey::from_secret(self.access_secret.as_bytes())
        ).map_err(|_| Error::Auth(AuthError::InvalidToken))?;

        let ref_claims = TokenClaims {
            sub: user.id,
            role: user.role.to_string(),
            exp: (now + Duration::seconds(self.refresh_expiry)).timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
            token_type: "ref".to_string()
        };

        let ref_token = encode(
            &Header::new(Algorithm::HS256),
            &ref_claims,
            &EncodingKey::from_secret(self.refresh_secret.as_bytes())
        ).map_err(|_| Error::Auth(AuthError::InvalidToken))?;

        Ok((acc_token, ref_token))
    }

    pub fn verify_access_token(&self, token: &str) -> Result<TokenClaims> {
        let validation = Validation::new(Algorithm::HS256);

        let token_data = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(self.access_secret.as_bytes()),
            &validation
        ).map_err(|e| {
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => Error::Auth(AuthError::TokenExpired),
                _ => Error::Auth(AuthError::InvalidToken)
            }
        })?;

        if token_data.claims.token_type != "acc" {
            return Err(Error::Auth(AuthError::InvalidToken));
        }

        Ok(token_data.claims)
    }

    pub fn verify_refresh_token(&self, token: &str) -> Result<TokenClaims> {
        let validation = Validation::new(Algorithm::HS256);

        let token_data = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(self.refresh_secret.as_bytes()),
            &validation
        ).map_err(|e| {
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => Error::Auth(AuthError::TokenExpired),
                _ => Error::Auth(AuthError::InvalidToken)
            }
        })?;

        if token_data.claims.token_type != "ref" {
            return Err(Error::Auth(AuthError::InvalidToken));
        }

        Ok(token_data.claims)
    }

    pub fn generate_access_token(&self, user: &User) -> Result<String> {
        let now = Utc::now();

        let acc_claims = TokenClaims {
            sub: user.id,
            role: user.role.to_string(),
            exp: (now + Duration::seconds(self.access_expiry)).timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
            token_type: "acc".to_string()
        };

        let acc_token = encode(
            &Header::new(Algorithm::HS256),
            &acc_claims,
            &EncodingKey::from_secret(self.access_secret.as_bytes())
        ).map_err(|_| Error::Auth(AuthError::InvalidToken))?;

        Ok(acc_token)
    }
}
