use actix_web::{web, HttpResponse, Responder};
use tokio_postgres::NoTls;
use std::env;
use rust_decimal::Decimal;
use std::str::FromStr;
use serde::Deserialize;
use actix_session::Session;
use crate::controllers::validation_controller;
use crate::controllers::csrf_controller;




#[derive(Deserialize)]
pub struct PriceForm {
    pub csrf_token: String,
    pub accommodation_id: i32,
    pub price: String,
}

pub async fn save_price(
    session: Session,
    form: web::Form<PriceForm>,
) -> impl Responder {

    // CSRF validation
    if !csrf_controller::verify_csrf_token(
        &session,
        &form.csrf_token,
    ) {
        return HttpResponse::Forbidden().finish();
    } 

    if let Err(err_code) = validation_controller::validate_price(&form.price) {
        return HttpResponse::SeeOther()
            .insert_header((
                actix_web::http::header::LOCATION,
                format!("/admin/prices?error={}", err_code),
            ))
            .finish();
    }

    let database_url =
        env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

    let (client, connection) =
        tokio_postgres::connect(
            &database_url,
            NoTls,
        )
        .await
        .expect("DB connect failed");

    actix_web::rt::spawn(async move {
        let _ = connection.await;
    });

    let price_decimal =
        Decimal::from_str(&form.price.trim())
            .expect("Invalid decimal format provided");

    client
        .execute(
            "
            UPDATE accommodation

            SET price_per_night = $1

            WHERE id = $2
            ",
            &[
                &price_decimal,
                &form.accommodation_id,
            ],
        )
        .await
        .expect("Update failed");

    HttpResponse::SeeOther()
        .insert_header((
            actix_web::http::header::LOCATION,
            "/admin/prices",
        ))
        .finish()
}