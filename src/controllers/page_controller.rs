


use actix_web::{web, HttpResponse, Responder, HttpRequest};
use actix_session::Session;
use askama::Template;
use crate::templates::*;
use serde::Deserialize;



// Helper function to extract language from cookie or default to "en"
fn get_lang(req: &HttpRequest) -> String {
    req.cookie("lang")
        .map(|c| c.value().to_string())
        .unwrap_or_else(|| "en".to_string())
}



// Pages
pub async fn public_home(req: HttpRequest, session: Session) -> impl Responder {
    let user_name: Option<String> = session.get("user_name").unwrap_or(None);
    let current_lang = get_lang(&req);

    let template = HomePublicTemplate { 
        user_name, 
        current_lang 
    };
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

pub async fn contact_page(req: HttpRequest, session: Session) -> impl Responder {
    let user_name: Option<String> = session.get("user_name").unwrap_or(None);
    let current_lang = get_lang(&req);

    let template = ContactTemplate {
        user_name,
        current_lang,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

pub async fn tos_page(req: HttpRequest, session: Session) -> impl Responder {
    let user_name: Option<String> = session.get("user_name").unwrap_or(None);
    let current_lang = get_lang(&req);

    let template = TosTemplate {
        user_name,
        current_lang,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}


// Public booking
pub async fn unavailable_dates_api(
    path: web::Path<i32>,
) -> HttpResponse {
    let accommodation_id = path.into_inner();

    let result = crate::controllers::db_controller::get_unavailable_dates(accommodation_id).await;

    match result {
        Ok(dates) => HttpResponse::Ok().json(dates),
        Err(_) => HttpResponse::InternalServerError().body("error"),
    }
}

#[derive(Deserialize)]
pub struct BookingParams {
    pub accommodation_id: Option<String>,
    pub error: Option<String>,
}

pub async fn public_booking1(
    req: HttpRequest,
    session: Session, 
    query: web::Query<BookingParams>
) -> impl Responder {
    let user_name: Option<String> = session.get("user_name").unwrap_or(None);
    let current_lang = get_lang(&req);
    let accommodation_id = query.accommodation_id.clone().unwrap_or_default();
    let error = query.error.clone().unwrap_or_default();

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

#[derive(Deserialize)]
pub struct BookingStep2Params {
    pub booking_id: Option<String>,
    pub lock_token: Option<String>,
    pub accommodation_id: Option<String>,
    pub check_in_date: Option<String>,
    pub check_out_date: Option<String>,
}

pub async fn public_booking2(
    req: HttpRequest,
    session: Session,
    query: web::Query<BookingStep2Params>,
) -> impl Responder {

    let user_name: Option<String> = session.get("user_name").unwrap_or(None);
    let current_lang = get_lang(&req);

    let template = PublicBooking2Template {
        user_name,
        current_lang,

        booking_id: query.booking_id.clone().unwrap_or_default(),
        lock_token: query.lock_token.clone().unwrap_or_default(),
        accommodation_id: query.accommodation_id.clone().unwrap_or_default(),
        check_in_date: query.check_in_date.clone().unwrap_or_default(),
        check_out_date: query.check_out_date.clone().unwrap_or_default(),
    };

    match template.render() {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html")
            .body(html),

        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}