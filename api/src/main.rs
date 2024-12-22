use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use sea_orm::{Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use utils::app_state::AppState;

mod entities;
mod routes;
mod utils;

#[derive(Debug, Serialize, Deserialize)]
struct MainError {
    pub message: String,
}

impl Responder for MainError {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::InternalServerError().json(self)
    }
}

#[actix_web::main]
async fn main() -> Result<(), MainError> {
    // Setting up logger
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    // Init logger
    dotenv::dotenv().ok();
    env_logger::init();

    // Get env variables
    let port = utils::constants::PORT.clone();
    let address = utils::constants::ADDRESS.clone();
    let db_url = utils::constants::DATABASE_URL.clone();

    // Establish database connection
    let db: DatabaseConnection = Database::connect(db_url).await.map_err(|err| MainError {
        message: err.to_string(),
    })?;

    // Booting up web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(Logger::default()) // Logger middleware
            .configure(routes::config) // Configure routes
    })
    .bind((address, port))
    .map_err(|err| MainError {
        message: err.to_string(),
    })?
    .run()
    .await
    .map_err(|err| MainError {
        message: err.to_string(),
    })
}
