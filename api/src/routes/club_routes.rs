use super::{controllers, middleware};
use actix_web::{middleware::from_fn, web};

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/club")
            .wrap(from_fn(middleware::auth_middleware::check_auth_middleware))
            .service(controllers::club_controller::get_user_club)
            .service(controllers::club_controller::create_club)
            .service(controllers::club_controller::leave_club)
            .service(controllers::club_controller::join_club),
    );
}
