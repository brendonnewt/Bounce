use actix_web::{middleware::Logger, App, HttpServer};

mod routes;
mod utils;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // Setting up logger
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv::dotenv().ok();
    env_logger::init();

    let port = utils::constants::PORT.clone();
    let address = utils::constants::ADDRESS.clone();

    // Booting up web server
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes::home_routes::config)
    })
    .bind((address, port))?
    .run()
    .await
}
