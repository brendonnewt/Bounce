use super::{controllers, middleware};
use actix_web::{middleware::from_fn, web};

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/user")
            .wrap(from_fn(middleware::auth_middleware::check_auth_middleware))
            .service(controllers::user_controller::get_user)
            .service(controllers::user_controller::update)
            .service(controllers::user_controller::reset_password),
    );
}
