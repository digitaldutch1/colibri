use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use std::env;
use tokio_postgres::NoTls;





// Confirm payment, generate invoice number and complete booking
pub async fn confirm_payment(path: web::Path<String>) -> impl Responder {
    
    // 1. Extract payment token from URL
    let payment_token = path.into_inner();

    // 2. Connect to database
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let (client, connection) = match tokio_postgres::connect(&database_url, NoTls).await {
        Ok(value) => value,
        Err(_) => return HttpResponse::InternalServerError().body("Database connection failed."),
    };

    actix_web::rt::spawn(async move {
        if let Err(error) = connection.await {
            eprintln!("Database connection error: {}", error);
        }
    });

     // 3. Find booking and customer data by payment token
    let booking_row = match client
        .query_opt(
            "
            SELECT 
                c.email,
                c.first_name,
                b.total_price::float8
            FROM booking b
            JOIN customer c ON c.id = b.customer_id
            WHERE b.payment_token = $1
            AND b.status = 'pending'
            AND b.locked_until IS NULL
            ",
            &[&payment_token],
        )
        .await
    {
        Ok(row) => row,
        Err(_) => return HttpResponse::InternalServerError().body("Booking lookup failed."),
    };

    let row = match booking_row {
        Some(row) => row,
        None => return HttpResponse::BadRequest().body("Payment could not be completed."),
    };

    let email: String = row.get(0);
    let first_name: String = row.get(1);
    let total_price: f64 = row.get(2);

    // 4. Generate invoice number
    let today = Utc::now().format("%Y%m%d").to_string();

    let count_row = match client
        .query_one(
            "
            SELECT COUNT(*) + 1
            FROM booking
            WHERE invoice_number IS NOT NULL
            ",
            &[],
        )
        .await
    {
        Ok(row) => row,
        Err(_) => return HttpResponse::InternalServerError().body("Invoice count failed."),
    };

    let invoice_count: i64 = count_row.get(0);

    let invoice_number = format!(
        "CC-{}-{:05}",
        today,
        invoice_count
    );

    // 5. Update booking status and store invoice number
    let updated = match client
        .execute(
            "
            UPDATE booking
            SET
                status = 'confirmed',
                invoice_number = $1
            WHERE payment_token = $2
            AND status = 'pending'
            AND locked_until IS NULL
            ",
            &[&invoice_number, &payment_token],
        )
        .await
    {
        Ok(count) => count,
        Err(_) => return HttpResponse::InternalServerError().body("Payment update failed."),
    };

    if updated == 0 {
        return HttpResponse::BadRequest().body("Payment could not be completed.");
    }

    // 6. Send payment confirmation email with invoice number
    match crate::controllers::email_controller::send_invoice_email(
        &email,
        &first_name,
        &invoice_number,
        &format!("{:.2}", total_price),
    ).await {
        Ok(_) => println!("Invoice email sent."),
        Err(error) => println!("Invoice email failed: {}", error),
    }

    // 7. Redirect to homepage with payment success alert
    HttpResponse::SeeOther()
        .insert_header((
            actix_web::http::header::LOCATION,
            format!(
                "/?payment=success&invoice={}",
                invoice_number
            ),
        ))
        .finish()
}