use ntex::web;
use crate::controllers::auth_controller;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(auth_controller::login))
            .route("/register", web::post().to(auth_controller::register))
            .route("/refresh", web::post().to(auth_controller::refresh_token))
            .route("/logout", web::post().to(auth_controller::logout))
    );
}