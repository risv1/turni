use actix_web::web;
use crate::controllers::misc_controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").route("/health", web::get().to(misc_controller::health)));
}
