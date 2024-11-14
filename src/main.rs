mod constants;
mod controllers;
mod routes;
mod utils;

use actix_web::{HttpServer, App, web, HttpResponse, Responder};
use crate::utils::load_env::load_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_config();

    let port = config.port;

    HttpServer::new(|| App::new().configure(routes::app_routes::config))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
