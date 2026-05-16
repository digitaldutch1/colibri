use actix_web::{web, HttpResponse, Responder, HttpRequest};
use actix_session::Session;
use askama::Template;
use chrono::NaiveDate;
use serde::Deserialize;
use std::env;
use tokio_postgres::NoTls;
use uuid::Uuid;
use actix_web::http::header;

use crate::templates::*;



// Helper function to extract language from cookie or default to "en"
fn get_lang(req: &HttpRequest) -> String {
    req.cookie("lang")
        .map(|c| c.value().to_string())
        .unwrap_or_else(|| "en".to_string())
}



// Redirect user back to booking step 1 when booking lock session is invalid or expired
fn redirect_with_error(message: &str, accommodation_id: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((
            header::LOCATION,
            format!(
                "/booking1?error={}&accommodation_id={}",
                message,
                accommodation_id
            ),
        ))
        .finish()
}



// Public booking step 1 form data struct
#[derive(Deserialize)]
pub struct PublicBookingStartForm {
    pub accommodation_id: String,
    pub check_in_date: String,
    pub check_out_date: String,
}

// Start public booking step 1, availability check and create temporary booking lock
pub async fn start_public_booking(form: web::Form<PublicBookingStartForm>) -> impl Responder {
    
    // 1. Parse and validate booking step 1 data
    let accommodation_id: i32 = match form.accommodation_id.parse() {
        Ok(value) => value,
        Err(_) => return HttpResponse::BadRequest().body("Invalid accommodation."),
    };

    let check_in_date = match NaiveDate::parse_from_str(&form.check_in_date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => return HttpResponse::BadRequest().body("Invalid check-in date."),
    };

    let check_out_date = match NaiveDate::parse_from_str(&form.check_out_date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => return HttpResponse::BadRequest().body("Invalid check-out date."),
    };

    if check_out_date <= check_in_date {
        return HttpResponse::BadRequest().body("Check-out date must be after check-in date.");
    }

     // 2. Calculate nights and temporary booking price
    let nights = (check_out_date - check_in_date).num_days();

    let price_per_night = match accommodation_id {
        1 => 300.0,
        2 => 200.0,
        3 => 100.0,
        _ => return HttpResponse::BadRequest().body("Invalid accommodation."),
    };

    let total_price = price_per_night * nights as f64;

     // 3. Connect to database
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

     // 4. Find available unit for selected accommodation and dates
    let unit_row = match client
        .query_opt(
            "
            SELECT u.id
            FROM unit u
            WHERE u.accommodation_id = $1
            AND NOT EXISTS (
                SELECT 1
                FROM booking b
                WHERE b.unit_id = u.id
                AND b.status != 'cancelled'
                AND (
                    b.locked_until IS NULL
                    OR b.locked_until > NOW()
                )
                AND b.check_in_date < $3
                AND b.check_out_date > $2
            )
            ORDER BY u.id
            LIMIT 1
            ",
            &[&accommodation_id, &check_in_date, &check_out_date],
        )
        .await
    {
        Ok(row) => row,
        Err(_) => return HttpResponse::InternalServerError().body("Availability check failed."),
    };

    let unit_id: i32 = match unit_row {
        Some(row) => row.get(0),
        None => return HttpResponse::BadRequest().body("No unit available for this period."),
    };

    // 5. Create lock token and temporary customer
    let lock_token = Uuid::new_v4().to_string();
    let temp_email = format!("lock-{}@temporary.local", lock_token);
    let customer_row = match client
        .query_one(
            "
            INSERT INTO customer (
                first_name,
                last_name,
                email
            )
            VALUES ('Temporary', 'Customer', $1)
            RETURNING id
            ",
            &[&temp_email],
        )
        .await
    {
        Ok(row) => row,
        Err(_) => return HttpResponse::InternalServerError().body("Temporary customer insert failed."),
    };

    let customer_id: i32 = customer_row.get(0);

    // 6. Create temporary pending booking lock for 5 minutes
    let booking_row = match client
        .query_one(
            "
            INSERT INTO booking (
                customer_id,
                accommodation_id,
                unit_id,
                check_in_date,
                check_out_date,
                total_price,
                status,
                payment_token,
                source,
                locked_until
            )
            VALUES (
                $1, $2, $3, $4, $5, ($6::float8)::numeric,
                'pending',
                $7,
                'colibri',
                NOW() + INTERVAL '5 minutes'
            )
            RETURNING id
            ",
            &[
                &customer_id,
                &accommodation_id,
                &unit_id,
                &check_in_date,
                &check_out_date,
                &total_price,
                &lock_token,
            ],
        )
        .await
    {
        Ok(row) => row,
        Err(_) => return HttpResponse::InternalServerError().body("Booking lock insert failed."),
    };

    let booking_id: i32 = booking_row.get(0);

    // 7. Build redirect URL for booking step 2
    let redirect_url = format!(
        "/booking2?booking_id={}&lock_token={}&accommodation_id={}&check_in_date={}&check_out_date={}",
        booking_id,
        lock_token,
        accommodation_id,
        form.check_in_date,
        form.check_out_date
    );

    // 8. Cleanup old expired booking locks
    let _ = crate::controllers::db_controller::cleanup_expired_booking_locks().await;

    // 9. Redirect user to booking step 2
    HttpResponse::SeeOther()
        .insert_header((header::LOCATION, redirect_url))
        .finish()
}



// Public booking step 2 form data struct
#[derive(Deserialize)]
pub struct PublicBookingForm {
    pub booking_id: String,
    pub lock_token: String,
    pub accommodation_id: String,
    pub check_in_date: String,
    pub check_out_date: String,
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub zip_code: String,
    pub city: String,
    pub phone: String,
    pub email: String,
    pub tos_accepted: Option<String>,
}

// Create new public booking
pub async fn create_public_booking(
    req: HttpRequest,
    session: Session,
    form: web::Form<PublicBookingForm>,
) -> impl Responder {

    // 1. Validate booking step 2 form data
    if form.tos_accepted.is_none() {
        return HttpResponse::BadRequest().body("Terms of service must be accepted.");
    }

    // 2. Parse booking step 1, accommodation and date values
    let booking_id: i32 = match form.booking_id.parse() {
        Ok(value) => value,
        Err(_) => return HttpResponse::BadRequest().body("Invalid booking id."),
    };

    let accommodation_id: i32 = match form.accommodation_id.parse() {
        Ok(value) => value,
        Err(_) => return HttpResponse::BadRequest().body("Invalid accommodation."),
    };

    let check_in_date = match NaiveDate::parse_from_str(&form.check_in_date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => return HttpResponse::BadRequest().body("Invalid check-in date."),
    };

    let check_out_date = match NaiveDate::parse_from_str(&form.check_out_date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => return HttpResponse::BadRequest().body("Invalid check-out date."),
    };

    if check_out_date <= check_in_date {
        return HttpResponse::BadRequest().body("Check-out date must be after check-in date.");
    }

    // 3. Calculate nights and total booking price
    let nights = (check_out_date - check_in_date).num_days();

    let price_per_night = match accommodation_id {
        1 => 300.0,
        2 => 200.0,
        3 => 100.0,
        _ => return HttpResponse::BadRequest().body("Invalid accommodation."),
    };

    let total_price = price_per_night * nights as f64;

    // 4. Connect to database
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

     // 5. Validate active booking lock from booking step 1
    let lock_row = match client
        .query_opt(
            "
            SELECT customer_id, unit_id
            FROM booking
            WHERE id = $1
            AND payment_token = $2
            AND locked_until > NOW()
            AND status = 'pending'
            ",
            &[&booking_id, &form.lock_token],
        )
        .await
    {
        Ok(row) => row,
        Err(_) => return HttpResponse::InternalServerError().body("Lock validation failed."),
    };

    let (temporary_customer_id, unit_id): (i32, i32) = match lock_row {
        Some(row) => (row.get(0), row.get(1)),
        None => {
            return redirect_with_error(
                "booking_session_expired",
                &form.accommodation_id,
            );
        }
    };

    // 6. Create or update real customer
    let customer_row = match client
        .query_one(
            "
            INSERT INTO customer (
                first_name,
                last_name,
                email,
                phone,
                address,
                postal_code,
                city
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (email)
            DO UPDATE SET
                first_name = EXCLUDED.first_name,
                last_name = EXCLUDED.last_name,
                phone = EXCLUDED.phone,
                address = EXCLUDED.address,
                postal_code = EXCLUDED.postal_code,
                city = EXCLUDED.city
            RETURNING id
            ",
            &[
                &form.first_name,
                &form.last_name,
                &form.email,
                &form.phone,
                &form.address,
                &form.zip_code,
                &form.city,
            ],
        )
        .await
    {
        Ok(row) => row,
        Err(_) => return HttpResponse::InternalServerError().body("Customer insert failed."),
    };

    let customer_id: i32 = customer_row.get(0);
    let cancel_token = Uuid::new_v4().to_string();

    // 7. Convert temporary booking lock into final booking
    let update_result = client
        .execute(
            "
            UPDATE booking
            SET
                customer_id = $1,
                accommodation_id = $2,
                unit_id = $3,
                check_in_date = $4,
                check_out_date = $5,
                total_price = ($6::float8)::numeric,
                cancel_token = $7,
                source = 'colibri',
                locked_until = NULL
            WHERE id = $8
            AND payment_token = $9
            AND locked_until > NOW()
            AND status = 'pending'
            ",
            &[
                &customer_id,
                &accommodation_id,
                &unit_id,
                &check_in_date,
                &check_out_date,
                &total_price,
                &cancel_token,
                &booking_id,
                &form.lock_token,
            ],
        )
        .await;

    match update_result {
        Ok(0) => {
            return redirect_with_error(
                "booking_session_expired",
                &form.accommodation_id,
            );
        }
        Ok(_) => {}
        Err(_) => return HttpResponse::InternalServerError().body("Booking update failed."),
    }

    // 8. Remove temporary lock customer
    let _ = client
        .execute(
            "
            DELETE FROM customer
            WHERE id = $1
            AND email LIKE 'lock-%@temporary.local'
            ",
            &[&temporary_customer_id],
        )
        .await;

    // 9. Prepare booking overview and email data   
    let user_name: Option<String> = session.get("user_name").unwrap_or(None);
    let current_lang = get_lang(&req);

    let accommodation = match accommodation_id {
        1 => "Chalet".to_string(),
        2 => "Tent".to_string(),
        3 => {
            if current_lang == "nl" {
                "Staanplaats".to_string()
            } else {
                "Pitch".to_string()
            }
        }
        _ => "Unknown".to_string(),
    };

    // 10. Send booking confirmation email
    match crate::controllers::email_controller::send_confirmation_email(
        &form.email,
        &form.first_name,
        &form.last_name,
        &form.phone,
        &form.address,
        &form.zip_code,
        &form.city,
        &accommodation,
        &check_in_date.format("%d-%m-%Y").to_string(),
        &check_out_date.format("%d-%m-%Y").to_string(),
        &form.lock_token,
        &cancel_token,
        nights,
        price_per_night,
        total_price,
    ).await {
        Ok(_) => println!("Booking confirmation email sent."),
        Err(error) => println!("Booking confirmation email failed: {}", error),
    }

     // 11. Render booking overview page
    let template = PublicBookingOverviewTemplate {
        user_name,
        current_lang,

        success: true,
        email: form.email.clone(),

        first_name: form.first_name.clone(),
        last_name: form.last_name.clone(),
        address: form.address.clone(),
        zip_code: form.zip_code.clone(),
        city: form.city.clone(),
        phone: form.phone.clone(),

        accommodation,
        check_in: check_in_date.format("%d-%m-%Y").to_string(),
        check_out: check_out_date.format("%d-%m-%Y").to_string(),

        nights,
        price_per_night: format!("{:.2}", price_per_night),
        total_price: format!("{:.2}", total_price),
        payment_token: form.lock_token.clone(),
    };

    match template.render() {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html")
            .body(html),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}



// Cleanup expired booking
pub async fn cleanup_expired_booking() -> impl Responder {
    let result = crate::controllers::db_controller::cleanup_expired_booking_locks().await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}



// Cancel public booking using cancel token from confirmation email
pub async fn cancel_booking(path: web::Path<String>) -> impl Responder {

    // 1. Extract cancel token from URL
    let cancel_token = path.into_inner();

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

    // 3. Find booking and customer data by cancel token
    let booking_row = match client
        .query_opt(
            "
            SELECT 
                c.email,
                c.first_name,
                COALESCE(b.invoice_number, 'Not paid yet')
            FROM booking b
            JOIN customer c ON c.id = b.customer_id
            WHERE b.cancel_token = $1
            AND b.status != 'cancelled'
            ",
            &[&cancel_token],
        )
        .await
    {
        Ok(row) => row,
        Err(_) => return HttpResponse::InternalServerError().body("Cancel lookup failed."),
    };

    let row = match booking_row {
        Some(row) => row,
        None => return HttpResponse::BadRequest().body("Booking could not be cancelled."),
    };

    let email: String = row.get(0);
    let first_name: String = row.get(1);
    let invoice_number: String = row.get(2);

    // 4. Update booking status to cancelled
    let updated = match client
        .execute(
            "
            UPDATE booking
            SET status = 'cancelled'
            WHERE cancel_token = $1
            AND status != 'cancelled'
            ",
            &[&cancel_token],
        )
        .await
    {
        Ok(count) => count,
        Err(_) => return HttpResponse::InternalServerError().body("Cancel update failed."),
    };

    if updated == 0 {
        return HttpResponse::BadRequest().body("Booking could not be cancelled.");
    }

    // 5. Send cancel confirmation email
    match crate::controllers::email_controller::send_cancel_confirmation_email(
        &email,
        &first_name,
        &invoice_number,
    ).await {
        Ok(_) => println!("Cancel confirmation email sent."),
        Err(error) => println!("Cancel confirmation email failed: {}", error),
    }

    // 6. Redirect to homepage with cancellation success alert
    HttpResponse::SeeOther()
        .insert_header((
            header::LOCATION,
            "/?error=booking_cancelled",
        ))
        .finish()
}