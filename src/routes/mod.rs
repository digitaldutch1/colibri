


use actix_web::web;
use crate::controllers::page_controller;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(page_controller::public_home))
        .route("/contact", web::get().to(page_controller::contact_page))
        .route("/tos", web::get().to(page_controller::tos_page))
        .route("/booking1", web::get().to(page_controller::public_booking1))
        .route("/booking2", web::get().to(page_controller::public_booking2));
}