use actix_web::{get, web, Responder};

use crate::utils::api_response;

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    api_response::ApiResponse::new(200, format!("Hello {name}!"))
}

#[get("/ping")]
pub async fn ping() -> impl Responder {
    api_response::ApiResponse::new(200, "Pong!".to_string())
}
