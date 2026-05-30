


use actix_web::{web, HttpResponse, Responder, HttpRequest};
use actix_session::Session;
use askama::Template;
use crate::templates::*;
use serde::Deserialize;
use tokio_postgres::NoTls;
use std::env;

// Login session time
const SESSION_TIMEOUT_HOURS: i64 = 8;


// Helper function to extract language from cookie or default to "en"
fn get_lang(req: &HttpRequest) -> String {
    req.cookie("lang")
        .map(|c| c.value().to_string())
        .unwrap_or_else(|| "en".to_string())
}


// Public pages
// Homepage query parameters for payment and cancel alerts
#[derive(Deserialize)]
pub struct HomeParams {
    pub payment: Option<String>,
    pub invoice: Option<String>,
    pub error: Option<String>,
}

// Render public homepage
pub async fn public_home(
    req: HttpRequest,
    session: Session,
    query: web::Query<HomeParams>,
) -> impl Responder {

     // 1. Get session user and selected language
    let user_name: Option<String> = session.get("user_name").unwrap_or(None);
    let current_lang = get_lang(&req);

    // 2. Read optional query parameters for homepage alerts
    let payment = query.payment.clone().unwrap_or_default();
    let invoice = query.invoice.clone().unwrap_or_default();
    let error = query.error.clone().unwrap_or_default();

    // 3. Render homepage template
    let template = HomePublicTemplate { 
        user_name, 
        current_lang,
        payment,
        invoice,
        error,
    };
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}



// Render public contact page
pub async fn contact_page(req: HttpRequest, session: Session) -> impl Responder {

    // 1. Get session user and selected language
    let user_name: Option<String> = session.get("user_name").unwrap_or(None);
    let current_lang = get_lang(&req);

    // 2. Render contact template
    let template = ContactTemplate {
        user_name,
        current_lang,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}



// Render terms of service page
pub async fn tos_page(req: HttpRequest, session: Session) -> impl Responder {
    
    // 1. Get session user and selected language
    let user_name: Option<String> = session.get("user_name").unwrap_or(None);
    let current_lang = get_lang(&req);

    // 2. Render terms of service template
    let template = TosTemplate {
        user_name,
        current_lang,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}



// API endpoint for unavailable booking dates
#[derive(Deserialize)]
pub struct UnavailableDatesQuery {
    pub exclude: Option<i32>,
}

pub async fn unavailable_dates_api(
    path: web::Path<i32>,
    query: web::Query<UnavailableDatesQuery>,
) -> HttpResponse {

    // 1. Extract accommodation id from URL
    let accommodation_id =
        path.into_inner();

    let exclude_booking_id =
        query.exclude.unwrap_or(0);

    // 2. Get unavailable dates from database    
    let result =
        crate::controllers::db_controller::get_unavailable_dates(
            accommodation_id,
            exclude_booking_id,
        ).await;

    // 3. Return unavailable dates as JSON    
    match result {

        Ok(dates) =>
            HttpResponse::Ok().json(dates),

        Err(_) =>
            HttpResponse::InternalServerError()
                .body("error"),
    }
}

// Booking step 1 query parameters
#[derive(Deserialize)]
pub struct BookingParams {
    pub accommodation_id: Option<String>,
    pub error: Option<String>,
}

// Render public booking step 1 page
pub async fn public_booking1(
    req: HttpRequest,
    session: Session, 
    query: web::Query<BookingParams>
) -> impl Responder {

    // 1. Get session user and selected language
    let user_name: Option<String> = session.get("user_name").unwrap_or(None);
    let current_lang = get_lang(&req);
    
    // 2. Read optional query parameters for selected accommodation and errors
    let accommodation_id = query.accommodation_id.clone().unwrap_or_default();
    let error = query.error.clone().unwrap_or_default();

    // 3. Render booking step 1 template
    let template = PublicBooking1Template {
        user_name,
        current_lang,
        accommodation_id,
        error,
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}



// Booking step 2 query parameters from booking lock
#[derive(Deserialize)]
pub struct BookingStep2Params {
    pub booking_id: Option<String>,
    pub lock_token: Option<String>,
    pub accommodation_id: Option<String>,
    pub check_in_date: Option<String>,
    pub check_out_date: Option<String>,
}

// Render public booking step 2 page
pub async fn public_booking2(
    req: HttpRequest,
    session: Session,
    query: web::Query<BookingStep2Params>,
) -> impl Responder {

     // 1. Get session user and selected language
    let user_name: Option<String> = session.get("user_name").unwrap_or(None);
    let current_lang = get_lang(&req);

    // 2. Render booking step 2 template with booking lock data
    let template = PublicBooking2Template {
        user_name,
        current_lang,

        booking_id: query.booking_id.clone().unwrap_or_default(),
        lock_token: query.lock_token.clone().unwrap_or_default(),
        accommodation_id: query.accommodation_id.clone().unwrap_or_default(),
        check_in_date: query.check_in_date.clone().unwrap_or_default(),
        check_out_date: query.check_out_date.clone().unwrap_or_default(),
    };

    // 3. Return rendered booking step 2 page
    match template.render() {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html")
            .body(html),

        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Booking overview query parameters from payment token
#[derive(Deserialize)]
pub struct PublicBookingOverviewParams {
    pub payment_token: Option<String>,
}

// Render public booking overview page
pub async fn public_booking_overview(
    req: HttpRequest,
    session: Session,
    query: web::Query<PublicBookingOverviewParams>,
) -> impl Responder {

    // Get selected language
    let current_lang = get_lang(&req);

    // Get session user
    let user_name: Option<String> =
        session.get("user_name").unwrap_or(None);

    // Database connection
    let database_url =
        env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

    let (client, connection) =
        tokio_postgres::connect(
            &database_url,
            NoTls
        )
        .await
        .unwrap();

    actix_web::rt::spawn(async move {

        if let Err(error) = connection.await {
            eprintln!("Database connection error: {}", error);
        }

    });

    // Get booking data
    let row = client
        .query_one(
            "
            SELECT
                c.email,
                c.first_name,
                c.last_name,
                c.address,
                c.postal_code,
                c.city,
                c.phone,
                a.name,
                TO_CHAR(b.check_in_date, 'DD-MM-YYYY'),
                TO_CHAR(b.check_out_date, 'DD-MM-YYYY'),
                (b.check_out_date - b.check_in_date),
                (b.total_price::float8 / (b.check_out_date - b.check_in_date)),
                b.total_price::float8,
                b.payment_token

            FROM booking b

            JOIN customer c
                ON c.id = b.customer_id

            JOIN accommodation a
                ON a.id = b.accommodation_id

            WHERE b.payment_token = $1
            ",
            &[&query.payment_token],
        )
        .await
        .unwrap();

    let template = PublicBookingOverviewTemplate {
        user_name,
        current_lang,
        success: true,
        email: row.get(0),
        first_name: row.get(1),
        last_name: row.get(2),
        address: row.get(3),
        zip_code: row.get(4),
        city: row.get(5),
        phone: row.get(6),
        accommodation: row.get(7),
        check_in: row.get(8),
        check_out: row.get(9),
        nights: row.get::<_, i32>(10) as i64,
        price_per_night:
            format!("{:.2}", row.get::<_, f64>(11)),

        total_price:
            format!("{:.2}", row.get::<_, f64>(12)),

        payment_token: row.get(13),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}





// Admin pages
// Render admin login page
pub async fn admin_login(
    req: HttpRequest,
) -> impl Responder {

    // 1. Get selected language
    let current_lang = get_lang(&req);

    // 2. Get optional error message from query string
    let error = req
        .query_string()
        .split('&')
        .find_map(|pair| {
            let mut parts = pair.split('=');

            match (parts.next(), parts.next()) {
                (Some("error"), Some(value)) => {
                    Some(value.replace("%20", " "))
                }
                _ => None,
            }
        });

    // 3. Render template
    let template = AdminLoginTemplate {
        user_name: None,
        current_lang,
        error,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

// Render admin homepage
pub async fn admin_home(
    req: HttpRequest,
    session: Session,
) -> impl Responder {

    
    let _ =
        crate::controllers::db_controller::expire_pending_bookings()
            .await;    

    // 1. Get selected language
    let current_lang = get_lang(&req);

    // 2. Check if user is logged in
    let logged_in = session
        .get::<bool>("logged_in")
        .unwrap_or(None)
        .unwrap_or(false);

    if !logged_in {
        return HttpResponse::Found()
            .append_header(("Location", "/admin/login"))
            .finish();
    }

    // 3. Check session timeout (8 hours)
    let login_at = session
        .get::<i64>("login_at")
        .unwrap_or(None)
        .unwrap_or(0);

    let now = chrono::Utc::now().timestamp();

    if now - login_at > SESSION_TIMEOUT_HOURS * 60 * 60 {

        // Session expired
        session.purge();

        return HttpResponse::Found()
            .append_header(("Location", "/admin/login?error=Sessie verlopen"))
            .finish();
    }

    // 4. Get session user
    let user_name: Option<String> =
        session.get("user_name").unwrap_or(None);

    // 5. Render admin homepage template
    let template = AdminHomeTemplate {
        user_name,
        current_lang,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

// Admin bookings read page query parameters 
#[derive(serde::Deserialize)]
pub struct AdminBookingParams {
    pub error: Option<String>,
    pub success: Option<String>,
}

// Render admin bookings read page
pub async fn admin_bookings_read(
    req: HttpRequest,
    session: Session,
    query: web::Query<AdminBookingParams>,
) -> impl Responder {

    // 1. Get selected language
    let current_lang = get_lang(&req);

    // 2. Check if user is logged in
    let logged_in = session
        .get::<bool>("logged_in")
        .unwrap_or(None)
        .unwrap_or(false);

    if !logged_in {
        return HttpResponse::Found()
            .append_header(("Location", "/admin/login"))
            .finish();
    }

    // 3. Get session user
    let user_name: Option<String> =
        session.get("user_name").unwrap_or(None);

    // 4. Get session role
    let user_role: String =
        session.get("role").unwrap_or(None).unwrap_or_default();

    // 5. Connect to database
    let database_url =
        env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

    let (client, connection) =
        tokio_postgres::connect(&database_url, NoTls)
            .await
            .expect("DB connect failed");

    actix_web::rt::spawn(async move {
        let _ = connection.await;
    });

    // 6. Query bookings
    let rows = client
        .query(
            "
            SELECT
                booking.id,
                customer.last_name,
                customer.email,
                customer.phone,
                unit.unit_code,
                booking.invoice_number,
                TO_CHAR(booking.check_in_date, 'DD-MM-YYYY'),
                TO_CHAR(booking.check_out_date, 'DD-MM-YYYY'),
                booking.status,
                booking.source

            FROM booking

            JOIN customer
                ON booking.customer_id = customer.id

            JOIN unit
                ON booking.unit_id = unit.id

            WHERE customer.first_name != 'Temporary'

            ORDER BY booking.id ASC
            ",
            &[],
        )
        .await
        .expect("Query failed");

    // 7. Convert query rows into BookingRow structs
    let bookings: Vec<BookingRow> = rows
        .iter()
        .map(|row| BookingRow {
            id: row.get(0),
            last_name: row.get(1),
            email: row.get(2),
            phone: row.get(3),
            unit_code: row.get(4),
            invoice_number: row.get(5),
            check_in: row.get(6),
            check_out: row.get(7),
            status: row.get(8),
            source: row.get(9),
        })
        .collect();

    // 8. Render template
    let template = AdminBookingsReadTemplate {
        user_name,
        current_lang,
        bookings,
        user_role,
        error: query.error.clone(),
        success: query.success.clone().unwrap_or_default(),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

// Render admin booking step 1 create page
pub async fn admin_booking1_create(
    req: HttpRequest,
    session: Session,
) -> impl Responder {

    // 1. Get selected language
    let current_lang = get_lang(&req);

    // 2. Check if user is logged in
    let logged_in = session
        .get::<bool>("logged_in")
        .unwrap_or(None)
        .unwrap_or(false);

    if !logged_in {

        return HttpResponse::Found()
            .append_header(("Location", "/admin/login"))
            .finish();
    }

    // 3. Get session user
    let user_name: Option<String> =
        session.get("user_name").unwrap_or(None);

    // 4. Render template
    let template = AdminBooking1CreateTemplate {
        user_name,
        current_lang,

        accommodation_id: String::new(),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

// Booking step 1 query parameters
#[derive(Deserialize)]
pub struct AdminBookingStep2Params {
    pub accommodation_id: Option<String>,
    pub check_in_date: Option<String>,
    pub check_out_date: Option<String>,
}

// Render admin booking step 2 create page
pub async fn admin_booking2_create(
    req: HttpRequest,
    session: Session,
    query: web::Query<AdminBookingStep2Params>,
) -> impl Responder {

    // 1. Get selected language
    let current_lang = get_lang(&req);

    // 2. Check if user is logged in
    let logged_in = session
        .get::<bool>("logged_in")
        .unwrap_or(None)
        .unwrap_or(false);

    if !logged_in {

        return HttpResponse::Found()
            .append_header(("Location", "/admin/login"))
            .finish();
    }

    // 3. Get session user
    let user_name: Option<String> =
        session.get("user_name").unwrap_or(None);

    // 4. Render template
    let template = AdminBooking2CreateTemplate {
        user_name,
        current_lang,
        accommodation_id:
            query.accommodation_id.clone().unwrap_or_default(),
        check_in_date:
            query.check_in_date.clone().unwrap_or_default(),
        check_out_date:
            query.check_out_date.clone().unwrap_or_default(),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}


#[derive(Deserialize)]
pub struct AdminBookingOverviewParams {
    pub payment_token: Option<String>,
}

// Render admin booking overview page
pub async fn admin_booking_overview(
    req: HttpRequest,
    session: Session,
    query: web::Query<AdminBookingOverviewParams>,
) -> impl Responder {

    // Get selected language
    let current_lang = get_lang(&req);

    // Check if user is logged in
    let logged_in = session
        .get::<bool>("logged_in")
        .unwrap_or(None)
        .unwrap_or(false);

    if !logged_in {

        return HttpResponse::Found()
            .append_header(("Location", "/admin/login"))
            .finish();
    }

    // Get session user
    let user_name: Option<String> =
        session.get("user_name").unwrap_or(None);

    // Database connection
    let database_url =
        env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

    let (client, connection) =
        tokio_postgres::connect(
            &database_url,
            NoTls
        )
        .await
        .unwrap();

    actix_web::rt::spawn(async move {

        if let Err(error) = connection.await {
            eprintln!("Database connection error: {}", error);
        }

    });

    // Get booking data
    let row = client
        .query_one(
            "
            SELECT
                c.email,
                c.first_name,
                c.last_name,
                c.address,
                c.postal_code,
                c.city,
                c.phone,
                a.name,
                TO_CHAR(b.check_in_date, 'DD-MM-YYYY'),
                TO_CHAR(b.check_out_date, 'DD-MM-YYYY'),
                (b.check_out_date - b.check_in_date),
                (b.total_price::float8 / (b.check_out_date - b.check_in_date)),
                b.total_price::float8,
                b.payment_token

            FROM booking b
            JOIN customer c
                ON c.id = b.customer_id
            JOIN accommodation a
                ON a.id = b.accommodation_id
            WHERE b.payment_token = $1
            ",
            &[&query.payment_token],
        )
        .await
        .unwrap();

    let template = AdminBookingOverviewTemplate {
        user_name,
        current_lang,
        success: true,
        email: row.get(0),
        first_name: row.get(1),
        last_name: row.get(2),
        address: row.get(3),
        zip_code: row.get(4),
        city: row.get(5),
        phone: row.get(6),
        accommodation: row.get(7),
        check_in: row.get(8),
        check_out: row.get(9),
        nights: row.get::<_, i32>(10) as i64,
        price_per_night:
            format!("{:.2}", row.get::<_, f64>(11)),
        total_price:
            format!("{:.2}", row.get::<_, f64>(12)),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

// Admin booking query parameters
#[derive(Deserialize)]
pub struct AdminBookingUpdatePath {
    pub error: Option<String>,
}

// Render admin booking update page
pub async fn admin_booking_update(
    req: HttpRequest,
    session: Session,
    path: web::Path<i32>,
    query: web::Query<AdminBookingUpdatePath>,
) -> impl Responder {

    // Get selected language
    let current_lang = get_lang(&req);

    // Check if user is logged in
    let logged_in = session
        .get::<bool>("logged_in")
        .unwrap_or(None)
        .unwrap_or(false);

    if !logged_in {
        return HttpResponse::Found()
            .append_header(("Location", "/admin/login"))
            .finish();
    }

    // Get session user
    let user_name: Option<String> =
        session.get("user_name").unwrap_or(None);

    // Database connection
    let database_url =
        env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

    let (client, connection) =
        tokio_postgres::connect(&database_url, NoTls)
            .await
            .unwrap();

    actix_web::rt::spawn(async move {
        if let Err(error) = connection.await {
            eprintln!("Database connection error: {}", error);
        }
    });

    // Get booking id
    let booking_id =
        path.into_inner();

    // Get booking data
    let row = client
        .query_one(
            "
            SELECT
                accommodation_id,
                TO_CHAR(check_in_date, 'YYYY-MM-DD'),
                TO_CHAR(check_out_date, 'YYYY-MM-DD')
            FROM booking
            WHERE id = $1
            ",
            &[&booking_id],
        )
        .await
        .unwrap();

    // Render template
    let template = AdminBooking1UpdateTemplate {
        user_name,
        current_lang,
        booking_id,
        accommodation_id:
            row.get::<_, i32>(0).to_string(),
        check_in_date:
            row.get(1),
        check_out_date:
            row.get(2),
        error:
            query.error.clone().unwrap_or_default(),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

// Admin booking update overview query parameters
#[derive(Deserialize)]
pub struct AdminBookingUpdateOverviewParams {
    pub booking_id: i32,
    pub accommodation_changed: bool,
    pub old_accommodation: String,
    pub new_accommodation: String,
    pub dates_changed: bool,
    pub old_check_in: String,
    pub old_check_out: String,
    pub new_check_in: String,
    pub new_check_out: String,
}

// Render admin booking update overview page
pub async fn admin_booking_update_overview(
    req: HttpRequest,
    session: Session,
    query: web::Query<AdminBookingUpdateOverviewParams>,
) -> impl Responder {

    // Get selected language
    let current_lang = get_lang(&req);

    // Check if user is logged in
    let logged_in = session
        .get::<bool>("logged_in")
        .unwrap_or(None)
        .unwrap_or(false);

    if !logged_in {
        return HttpResponse::Found()
            .append_header(("Location", "/admin/login"))
            .finish();
    }

    // Get session user
    let user_name: Option<String> =
        session.get("user_name").unwrap_or(None);

    // Database connection
    let database_url =
        env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

    let (client, connection) =
        tokio_postgres::connect(&database_url, NoTls)
            .await
            .unwrap();

    actix_web::rt::spawn(async move {
        if let Err(error) = connection.await {
            eprintln!("Database connection error: {}", error);
        }
    });

    // Get updated booking data
    let row = client
        .query_one(
            "
            SELECT
                accommodation.name,
                TO_CHAR(booking.check_in_date, 'DD-MM-YYYY'),
                TO_CHAR(booking.check_out_date, 'DD-MM-YYYY'),
                (booking.check_out_date - booking.check_in_date),
                (booking.total_price::float8 / (booking.check_out_date - booking.check_in_date)),
                booking.total_price::float8
            FROM booking
            JOIN accommodation
                ON booking.accommodation_id = accommodation.id
            WHERE booking.id = $1
            ",
            &[&query.booking_id],
        )
        .await
        .unwrap();

    // Render template
    let template = AdminBookingUpdateOverviewTemplate {
        user_name,
        current_lang,
        success: true,
        accommodation_changed:
            query.accommodation_changed,
        old_accommodation:
            query.old_accommodation.clone(),
        new_accommodation:
            query.new_accommodation.clone(),
        dates_changed:
            query.dates_changed,
        old_check_in:
            query.old_check_in.clone(),
        old_check_out:
            query.old_check_out.clone(),
        
        new_check_in:
            query.new_check_in.clone(),
        new_check_out:
            query.new_check_out.clone(),
        accommodation: row.get(0),
        check_in: row.get(1),
        check_out: row.get(2),
        nights: row.get::<_, i32>(3) as i64,
        price_per_night:
            format!("{:.2}", row.get::<_, f64>(4)),
        total_price:
            format!("{:.2}", row.get::<_, f64>(5)),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

// Admin booking status path
#[derive(Deserialize)]
pub struct AdminBookingStatusPath {
    pub id: i32,
}

// Render admin booking status page
pub async fn admin_booking_status(
    req: HttpRequest,
    session: Session,
    path: web::Path<AdminBookingStatusPath>,
) -> impl Responder {

    // Get selected language
    let current_lang = get_lang(&req);

    // Check if user is logged in
    let logged_in = session
        .get::<bool>("logged_in")
        .unwrap_or(None)
        .unwrap_or(false);

    if !logged_in {

        return HttpResponse::Found()
            .append_header(("Location", "/admin/login"))
            .finish();
    }

    // Get session user
    let user_name: Option<String> =
        session.get("user_name").unwrap_or(None);

    // Database connection
    let database_url =
        env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

    let (client, connection) =
        tokio_postgres::connect(&database_url, NoTls)
            .await
            .unwrap();

    actix_web::rt::spawn(async move {

        if let Err(error) = connection.await {
            eprintln!("Database connection error: {}", error);
        }
    });

    // Get booking status
    let row = client
        .query_one(
            "
            SELECT status
            FROM booking
            WHERE id = $1
            ",
            &[&path.id],
        )
        .await
        .unwrap();

    let current_status: String =
        row.get(0);

    // Render template
    let template = AdminBookingStatusTemplate {
        user_name,
        current_lang,
        booking_id:
            path.id,
        current_status,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}





// Admin customer
// Admin customers query parameters
#[derive(Deserialize)]
pub struct AdminCustomerParams {
    pub success: Option<String>,
    pub last_name: Option<String>,
}

// Render admin customers page
pub async fn admin_customers_read(
    req: HttpRequest,
    session: Session,
    _query: web::Query<AdminCustomerParams>,
) -> impl Responder {

    // Get selected language
    let current_lang = get_lang(&req);

    // Check if user is logged in
    let logged_in = session
        .get::<bool>("logged_in")
        .unwrap_or(None)
        .unwrap_or(false);

    if !logged_in {

        return HttpResponse::Found()
            .append_header(("Location", "/admin/login"))
            .finish();
    }

    // Get session user
    let user_name: Option<String> =
        session.get("user_name").unwrap_or(None);

    let user_role: String =
        session.get::<String>("user_role")
            .unwrap_or(None)
            .unwrap_or_default();

    // Get all customers
    let customers =
        crate::controllers::db_controller::get_all_customers().await;

    // Render template
    let template = AdminCustomerReadTemplate {
        user_name,
        current_lang,
        customers,
        user_role,

        success:
            _query.success.clone().unwrap_or_default(),

        success_last_name:
            _query.last_name.clone().unwrap_or_default(),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

// Render admin customer create page
pub async fn admin_customer_create(
    req: HttpRequest,
    session: Session,
) -> impl Responder {

    // Get selected language
    let current_lang = get_lang(&req);

    // Check if user is logged in
    let logged_in = session
        .get::<bool>("logged_in")
        .unwrap_or(None)
        .unwrap_or(false);

    if !logged_in {

        return HttpResponse::Found()
            .append_header(("Location", "/admin/login"))
            .finish();
    }

    // Get session user
    let user_name: Option<String> =
        session.get("user_name").unwrap_or(None);

    // Render template
    let template = AdminCustomerCreateTemplate {
        user_name,
        current_lang,
        first_name: String::new(),
        last_name: String::new(),
        email: String::new(),
        phone: String::new(),
        address: String::new(),
        postal_code: String::new(),
        city: String::new(),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

// Admin customers update query parameters
#[derive(Deserialize)]
pub struct CustomerPath {
    pub id: i32,
}

// Render admin customer update page
pub async fn admin_customer_update(
    req: HttpRequest,
    session: Session,
    path: web::Path<CustomerPath>,
) -> impl Responder {

    // Get selected language
    let current_lang = get_lang(&req);

    // Check if user is logged in
    let logged_in = session
        .get::<bool>("logged_in")
        .unwrap_or(None)
        .unwrap_or(false);

    if !logged_in {

        return HttpResponse::Found()
            .append_header(("Location", "/admin/login"))
            .finish();
    }

    // Get session user
    let user_name: Option<String> =
        session.get("user_name").unwrap_or(None);

    // Database connection
    let database_url =
        env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

    let (client, connection) =
        tokio_postgres::connect(&database_url, NoTls)
            .await
            .unwrap();

    actix_web::rt::spawn(async move {

        if let Err(error) = connection.await {
            eprintln!("Database connection error: {}", error);
        }
    });

    // Get customer
    let row = client
        .query_one(
            "
            SELECT
                first_name,
                last_name,
                email,
                phone,
                address,
                postal_code,
                city

            FROM customer

            WHERE id = $1
            ",
            &[&path.id],
        )
        .await
        .unwrap();

    // Render template
    let template = AdminCustomerUpdateTemplate {
        user_name,
        current_lang,
        customer_id:
            path.id,
        first_name:
            row.get::<_, String>(0),
        last_name:
            row.get::<_, String>(1),
        email:
            row.get::<_, String>(2),
        phone:
            row.get::<_, Option<String>>(3)
                .unwrap_or_default(),
        address:
            row.get::<_, Option<String>>(4)
                .unwrap_or_default(),
        postal_code:
            row.get::<_, Option<String>>(5)
                .unwrap_or_default(),
        city:
            row.get::<_, Option<String>>(6)
                .unwrap_or_default(),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

// Admin staff query parameters
#[derive(Deserialize)]
pub struct AdminStaffParams {
    pub success: Option<String>,
    pub last_name: Option<String>,
}

// Render admin staff page
pub async fn admin_staff_read(
    req: HttpRequest,
    session: Session,
    _query: web::Query<AdminStaffParams>,
) -> impl Responder {

    // Get selected language
    let current_lang = get_lang(&req);

    // Check if user is logged in
    let logged_in = session
        .get::<bool>("logged_in")
        .unwrap_or(None)
        .unwrap_or(false);

    if !logged_in {

        return HttpResponse::Found()
            .append_header(("Location", "/admin/login"))
            .finish();
    }

    // Get session user
    let user_name: Option<String> =
        session.get("user_name").unwrap_or(None);

    let user_role: String =
        session.get::<String>("user_role")
            .unwrap_or(None)
            .unwrap_or_default();

    // Get all staff
    let staff =
        crate::controllers::db_controller::get_all_staff().await;

    // Render template
    let template = AdminStaffReadTemplate {
        user_name,
        current_lang,
        staff,
        user_role,
        success:
            _query.success.clone().unwrap_or_default(),
        success_last_name:
            _query.last_name.clone().unwrap_or_default(),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

// Render admin staff create page
pub async fn admin_staff_create(
    req: HttpRequest,
    session: Session,
) -> impl Responder {

    // Get selected language
    let current_lang = get_lang(&req);

    // Check if user is logged in
    let logged_in = session
        .get::<bool>("logged_in")
        .unwrap_or(None)
        .unwrap_or(false);

    if !logged_in {

        return HttpResponse::Found()
            .append_header(("Location", "/admin/login"))
            .finish();
    }

    // Get session user
    let user_name: Option<String> =
        session.get("user_name").unwrap_or(None);

    let user_role: String =
        session.get::<String>("user_role")
            .unwrap_or(None)
            .unwrap_or_default();

    if user_role != "admin" {

        return HttpResponse::Found()
            .append_header(("Location", "/admin/staff"))
            .finish();
    }       

    // Render template
    let template = AdminStaffCreateTemplate {
        user_name,
        current_lang,
        first_name: String::new(),
        last_name: String::new(),
        email: String::new(),
        password: String::new(),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[derive(Deserialize)]
pub struct StaffPath {
    pub id: i32,
}

// Render admin staff update page
pub async fn admin_staff_update(
    req: HttpRequest,
    session: Session,
    path: web::Path<StaffPath>,
) -> impl Responder {

    // Get selected language
    let current_lang = get_lang(&req);

    // Check if user is logged in
    let logged_in = session
        .get::<bool>("logged_in")
        .unwrap_or(None)
        .unwrap_or(false);

    if !logged_in {

        return HttpResponse::Found()
            .append_header(("Location", "/admin/login"))
            .finish();
    }

    // Get session user
    let user_name: Option<String> =
        session.get("user_name").unwrap_or(None);

    let user_role: String =
        session.get::<String>("user_role")
            .unwrap_or(None)
            .unwrap_or_default();

    if user_role != "admin" {

        return HttpResponse::Found()
            .append_header(("Location", "/admin/staff"))
            .finish();
    }

    // Database connection
    let database_url =
        env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

    let (client, connection) =
        tokio_postgres::connect(&database_url, NoTls)
            .await
            .unwrap();

    actix_web::rt::spawn(async move {

        if let Err(error) = connection.await {
            eprintln!("Database connection error: {}", error);
        }
    });

    // Get user
    let row = client
        .query_one(
            "
            SELECT
                first_name,
                last_name,
                email,
                role

            FROM \"user\"
            WHERE id = $1
            ",
            &[&path.id],
        )
        .await
        .unwrap();

    // Render template
    let template = AdminStaffUpdateTemplate {
        user_name,
        current_lang,
        user_id:
            path.id,
        first_name:
            row.get::<_, String>(0),
        last_name:
            row.get::<_, String>(1),
        email:
            row.get::<_, String>(2),
        role:
            row.get::<_, String>(3),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}