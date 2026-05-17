


use actix_web::{web, HttpResponse, Responder, HttpRequest};
use actix_session::Session;
use askama::Template;
use crate::templates::*;
use serde::Deserialize;

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
pub async fn unavailable_dates_api(
    path: web::Path<i32>,
) -> HttpResponse {

    // 1. Extract accommodation id from URL
    let accommodation_id = path.into_inner();

    // 2. Get unavailable dates from database
    let result = crate::controllers::db_controller::get_unavailable_dates(accommodation_id).await;

    // 3. Return unavailable dates as JSON
    match result {
        Ok(dates) => HttpResponse::Ok().json(dates),
        Err(_) => HttpResponse::InternalServerError().body("error"),
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