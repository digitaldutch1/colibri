


use actix_web::web;
use crate::controllers::*;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        // Public pages
        .route("/", web::get().to(page_controller::public_home))
        .route("/contact", web::get().to(page_controller::contact_page))
        .route("/tos", web::get().to(page_controller::tos_page))
        .route("/booking1", web::get().to(page_controller::public_booking1))
        .route("/booking2", web::get().to(page_controller::public_booking2))
        
        // Public booking, payment and cancellation routes
        .route("/booking/create", web::post().to(booking_controller::create_public_booking))
        .route("/booking/start", web::post().to(booking_controller::start_public_booking))
        .route("/api/unavailable/{id}", web::get().to(page_controller::unavailable_dates_api))
        .route("/payment/{token}/confirm", web::get().to(payment_controller::confirm_payment))
        .route("/cancel/{token}", web::get().to(booking_controller::cancel_booking))
        .route("/booking/cleanup-expired", web::post().to(booking_controller::cleanup_expired_booking))
        
        // Admin pages
        .route("/admin", web::get().to(page_controller::admin_home));
}