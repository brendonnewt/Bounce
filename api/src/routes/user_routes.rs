use super::controllers;
use actix_web::web;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/users")
            .service(controllers::user_controller::greet)
            .service(controllers::user_controller::ping),
    );
}
