


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
        .route("/booking-overview", web::get().to(page_controller::public_booking_overview))

        // Public booking, payment and cancellation routes
        .route("/booking/create", web::post().to(booking_controller::create_public_booking))
        .route("/booking/start", web::post().to(booking_controller::start_public_booking))
        .route("/api/unavailable/{id}", web::get().to(page_controller::unavailable_dates_api))
        .route("/payment/{token}/confirm", web::get().to(payment_controller::confirm_payment))
        .route("/cancel/{token}", web::get().to(booking_controller::cancel_booking))
        .route("/booking/cleanup-expired", web::post().to(booking_controller::cleanup_expired_booking))

        // Admin pages
        .route("/admin/login", web::get().to(page_controller::admin_login))
        .route("/admin/login", web::post().to(login_controller::login_admin))
        .route("/logout", web::get().to(login_controller::logout_admin))
        .route("/admin", web::get().to(page_controller::admin_home))
        
        .route("/admin/bookings", web::get().to(page_controller::admin_bookings_read))
        .route("/admin/booking1", web::get().to(page_controller::admin_booking1_create))
        .route("/admin/booking2", web::get().to(page_controller::admin_booking2_create))
        .route("/admin/booking/create", web::post().to(booking_controller::create_admin_booking))
        .route("/admin/booking-overview", web::get().to(page_controller::admin_booking_overview))
        
        .route("/admin/booking/update/{id}", web::get().to(page_controller::admin_booking_update))
        .route("/admin/booking/update/save", web::post().to(booking_controller::update_admin_booking))
        .route("/admin/booking/update-overview", web::get().to(page_controller::admin_booking_update_overview))
        .route("/admin/booking/status/{id}", web::get().to(page_controller::admin_booking_status))
        .route("/admin/booking/status/save", web::post().to(booking_controller::save_admin_booking_status))
        .route("/admin/booking/delete", web::post().to(booking_controller::delete_booking))
        
        .route("/admin/customers", web::get().to(page_controller::admin_customers_read))
        .route("/admin/customer/create", web::get().to(page_controller::admin_customer_create))
        .route("/admin/customer/create", web::post().to(customer_controller::create_customer))
        .route("/admin/customer/update/{id}", web::get().to(page_controller::admin_customer_update))
        .route("/admin/customer/update/save", web::post().to(customer_controller::update_customer))
        .route("/admin/customer/delete", web::post().to(customer_controller::delete_customer))
        
        .route("/admin/staff", web::get().to(page_controller::admin_staff_read))
        .route("/admin/staff/create", web::get().to(page_controller::admin_staff_create))
        .route("/admin/staff/create", web::post().to(admin_staff_controller::create_staff))
        .route("/admin/staff/update/{id}", web::get().to(page_controller::admin_staff_update))
        .route("/admin/staff/update/save", web::post().to(admin_staff_controller::update_staff))
        .route("/admin/staff/delete", web::post().to(admin_staff_controller::delete_staff))
        
        .route("/admin/prices", web::get().to(page_controller::admin_prices))
        .route("/admin/prices/save", web::post().to(price_controller::save_price))

        .route("/admin/bookings/{id}/print",web::get().to(print_controller::booking_print),);

}