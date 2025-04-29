use std::sync::Arc;

use ntex::http::header;
use ntex::service::{Service, ServiceCtx};
use ntex::web;

use crate::app::AppState;
use crate::error::Error;
use crate::services::token_service::TokenClaims;

pub struct Auth;

impl<S> ntex::service::Middleware<S> for Auth {
    type Service = AuthMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        AuthMiddleware { service }
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, Err> Service<web::WebRequest<Err>> for AuthMiddleware<S>
where
    S: Service<web::WebRequest<Err>, Response = web::WebResponse, Error = web::Error>,
    Err: web::ErrorRenderer,
{
    type Response = web::WebResponse;
    type Error = web::Error;

    ntex::forward_ready!(service);

    async fn call(&self, req: web::WebRequest<Err>, ctx: ServiceCtx<'_, Self>) -> Result<Self::Response, Self::Error> {
        let path = req.path();
        if path.contains("/auth/login") || path.contains("/auth/register") || path.contains("/auth/refresh") {
            return ctx.call(&self.service, req).await;
        }

        let auth_header = req.headers().get(header::AUTHORIZATION);
        
        if auth_header.is_none() {
            return Err(Error::ForbiddenError.into());
        }
        
        let auth_header = auth_header.unwrap().to_str().unwrap_or_default();
        
        if !auth_header.starts_with("Bearer ") {
            return Err(Error::ForbiddenError.into());
        }
        
        let token = auth_header.trim_start_matches("Bearer ").trim();
        
        if token.is_empty() {
            return Err(Error::ForbiddenError.into());
        }
        
        let state = req.app_state::<Arc<AppState>>().unwrap();
        
        match state.token_service.verify_access_token(token) {
            Ok(claims) => {
                req.extensions_mut().insert(claims);
                let res = ctx.call(&self.service, req).await?;
                Ok(res)
            }
            Err(e) => {
                Err(e.into())
            }
        }
    }
}

pub fn get_user_id(req: &web::HttpRequest) -> Option<i32> {
    req.extensions().get::<TokenClaims>().map(|claims| claims.sub)
}

pub fn get_user_role(req: &web::HttpRequest) -> Option<String> {
    req.extensions().get::<TokenClaims>().map(|claims| claims.role.clone())
}

pub trait UserInfo {
    fn user_id(&self) -> Option<i32>;
    fn user_role(&self) -> Option<String>;
}

impl UserInfo for web::HttpRequest {
    fn user_id(&self) -> Option<i32> {
        get_user_id(self)
    }
    
    fn user_role(&self) -> Option<String> {
        get_user_role(self)
    }
}
