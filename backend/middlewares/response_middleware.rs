use std::future::poll_fn;
use std::borrow::Cow;

use ntex::http::body::{Body, MessageBody, ResponseBody};
use ntex::http::header;
use ntex::service::{Middleware, Service, ServiceCtx};
use ntex::util::BytesMut;
use ntex::web;
use serde_json::Value;

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

pub struct Response;

impl<S> Middleware<S> for Response {
    type Service = ResponseMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        ResponseMiddleware { service }
    }
}

pub struct ResponseMiddleware<S> {
    service: S,
}

impl<S> ResponseMiddleware<S> {
    async fn extract_body(&self, res: &mut web::WebResponse) -> Cow<'static, str> {
        let mut body = res.take_body();
        let mut buf = BytesMut::with_capacity(1024);

        while let Some(chunk) = poll_fn(|cx| body.poll_next_chunk(cx)).await { 
            match chunk {
                Ok(bytes) => buf.extend_from_slice(&bytes),
                Err(e) => {
                    buf.extend_from_slice(e.to_string().as_bytes());
                    break;
                }
            }
        }

        match String::from_utf8(buf.freeze().to_vec()) {
            Ok(s) => Cow::Owned(s),
            Err(_) => Cow::Borrowed("Invalid UTF-8 content")
        }
    }
    
    fn format_response(&self, status: ntex::http::StatusCode, body_str: &str) -> Cow<'static, str> {
        if !status.is_success() {
            return self.format_error_response(body_str);
        }
        
        self.format_success_response(body_str)
    }
    
    fn format_error_response(&self, body_str: &str) -> Cow<'static, str> {
        let error_response = ErrorResponse::new(body_str);
        
        match serde_json::to_string(&error_response) {
            Ok(json) => Cow::Owned(json),
            Err(_) => Cow::Borrowed("{\"success\":false,\"message\":\"Oops, seems like our server had a little bit of hickups, Please try again later\"}")
        }
    }
    
    fn format_success_response(&self, body_str: &str) -> Cow<'static, str> {
        let trimmed = body_str.trim();
        
        if trimmed.starts_with('{') || trimmed.starts_with('[') {
            if let Ok(data) = serde_json::from_str::<Value>(body_str) {
                let success_response = SuccessResponse::new(data);
                
                if let Ok(json) = serde_json::to_string(&success_response) {
                    return Cow::Owned(json);
                }
                return Cow::Borrowed("{\"success\":true,\"data\":{}}");
            }
        }
        
        Cow::Owned(body_str.to_owned())
    }
}

impl<S, Err> Service<web::WebRequest<Err>> for ResponseMiddleware<S> 
where
    S: Service<web::WebRequest<Err>, Response = web::WebResponse, Error = web::Error>,
    Err: web::ErrorRenderer
{
    type Response = web::WebResponse;
    type Error = web::Error;

    ntex::forward_ready!(service);

    async fn call(&self, req: web::WebRequest<Err>, ctx: ServiceCtx<'_, Self>) -> Result<Self::Response, Self::Error> {
        let mut res = ctx.call(&self.service, req).await?;
        
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json; charset=utf-8")
        );
        
        let status = res.status();
        let body_str = self.extract_body(&mut res).await;
        
        let json_body = self.format_response(status, &body_str);
        
        res = res.map_body(move |head, _| {
            head.headers.insert(header::CONTENT_LENGTH, json_body.len().into());
            ResponseBody::Body(Body::from(json_body.into_owned()))
        });

        Ok(res)
    }
}