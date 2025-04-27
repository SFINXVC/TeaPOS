use std::future::poll_fn;

use ntex::http::body::{Body, MessageBody, ResponseBody};
use ntex::service::{Middleware, Service, ServiceCtx};
use ntex::util::BytesMut;
use ntex::web;
use ntex::http::header;
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
    async fn extract_body(&self, res: &mut web::WebResponse) -> String {
        let mut body = res.take_body();
        let mut buf = BytesMut::new();

        while let Some(chunk) = poll_fn(|cx| body.poll_next_chunk(cx)).await { 
            match chunk {
                Ok(bytes) => buf.extend_from_slice(&bytes),
                Err(e) => {
                    buf.extend_from_slice(e.to_string().as_bytes());
                    break;
                }
            }
        }

        String::from_utf8_lossy(&buf).to_string()
    }
    
    fn format_response(&self, status: ntex::http::StatusCode, body_str: &str) -> String {
        if !status.is_success() {
            return self.format_error_response(body_str);
        }
        
        self.format_success_response(body_str)
    }
    
    fn format_error_response(&self, body_str: &str) -> String {
        let error_response = ErrorResponse::new(body_str.to_string());
        
        serde_json::to_string(&error_response)
            .unwrap_or_else(|_| "{\"success\":false,\"message\":\"Oops, seems like our server had a little bit of hickups, Please try again later\"}".to_string())
    }
    
    fn format_success_response(&self, body_str: &str) -> String {
        match serde_json::from_str::<Value>(body_str) {
            Ok(data) => {
                // Create a new SuccessResponse to ensure field order
                let success_response = SuccessResponse::new(data);
                
                // Use a custom serializer with preserve_order feature
                serde_json::to_string(&success_response)
                    .unwrap_or_else(|_| "{\"success\":true,\"data\":{}}".to_string())
            },
            Err(_) => body_str.to_string()
        }
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
            ResponseBody::Body(Body::from(json_body))
        });

        Ok(res)
    }
}