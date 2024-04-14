use crate::controller::users::*;
use actix_web::web;

pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login))
            .route("/register", web::post().to(register))
            .route("/logout", web::post().to(logout)),
    );
}
