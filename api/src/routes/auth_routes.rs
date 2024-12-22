use super::controllers;
use actix_web::web;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/auth")
            .service(controllers::auth_controller::login)
            .service(controllers::auth_controller::register_athlete),
    );
}
