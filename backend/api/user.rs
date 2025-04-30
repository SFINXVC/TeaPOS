use ntex::web;
use crate::controllers::user_controller;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            // Current user endpoints
            .route("/me", web::get().to(user_controller::get_current_user))
            .route("/me/details", web::get().to(user_controller::get_user_details))
            .route("/me/update", web::put().to(user_controller::update_user))
            .route("/me/password", web::put().to(user_controller::update_password))
            
            // Admin-only endpoints
            .route("", web::get().to(user_controller::list_users))
            .route("/{id}", web::get().to(user_controller::admin_get_user))
            .route("/{id}", web::put().to(user_controller::admin_update_user))
    );
}