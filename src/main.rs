mod constants;
mod controllers;
mod database;
mod routes;
mod utils;

use actix_web::{HttpServer, App, web::Data};
use crate::utils::load_env::load_config;
use crate::database::db::establish_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_config();

    let port = config.port;

    let pool =  establish_connection(&config.database_url);
    
    if pool.get().is_ok() {
        println!("Database connection established! 🚀");
    }

    println!("Server running on port: {} 🥳", port);

    HttpServer::new(move || App::new().app_data(Data::new(pool.clone())).configure(routes::app_routes::config))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
