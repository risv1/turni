mod constants;
mod controllers;
mod routes;
mod utils;

use actix_web::{HttpServer, App};
use crate::utils::load_env::load_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_config();

    let port = config.port;
    println!("Server running on port: {}", port);

    HttpServer::new(|| App::new().configure(routes::app_routes::config))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
