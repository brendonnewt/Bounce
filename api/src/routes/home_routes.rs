use super::controllers;
use actix_web::web;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/home")
            .service(controllers::home_handler::greet)
            .service(controllers::home_handler::ping),
    );
}
