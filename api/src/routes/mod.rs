pub mod controllers;
pub mod middleware;
pub mod services;

pub mod auth_routes;
pub mod user_routes;

use actix_web::web;

// Config for each route
pub fn config(config: &mut web::ServiceConfig) {
    user_routes::config(config);
    auth_routes::config(config);
}
