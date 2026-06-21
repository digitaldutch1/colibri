use crate::templates::AdminBookingPrintTemplate;
use crate::templates::BookingPrintRow;
use actix_web::{web, HttpResponse, Responder};
use askama::Template;
use tokio_postgres::NoTls;
use std::env;




pub async fn booking_print(
    path: web::Path<i32>,
) -> impl Responder {

    let booking_id = path.into_inner();

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

    let row = client
        .query_one(
            "
            SELECT
                booking.id,
                customer.first_name,
                customer.last_name,
                customer.email,
                customer.phone,
                unit.unit_code,
                accommodation.name,
                booking.invoice_number,
                TO_CHAR(booking.check_in_date,'DD-MM-YYYY'),
                TO_CHAR(booking.check_out_date,'DD-MM-YYYY'),
                booking.status

            FROM booking

            JOIN customer
                ON booking.customer_id = customer.id

            JOIN unit
                ON booking.unit_id = unit.id

            JOIN accommodation
                ON unit.accommodation_id = accommodation.id

            WHERE booking.id = $1
            ",
            &[&booking_id],
        )
        .await
        .expect("Booking not found");

    let booking = BookingPrintRow {
        id: row.get(0),
        first_name: row.get(1),
        last_name: row.get(2),
        email: row.get(3),
        phone: row.get(4),
        unit_code: row.get(5),
        accommodation_name: row.get(6),
        invoice_number: row.get(7),
        check_in: row.get(8),
        check_out: row.get(9),
        status: row.get(10),
    };

    let template =
        AdminBookingPrintTemplate {
            booking,
        };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}