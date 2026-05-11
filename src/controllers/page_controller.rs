


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



#[derive(Deserialize)]
pub struct BookingParams {
    pub accommodation_id: Option<String>,
}

pub async fn public_booking1(
    req: HttpRequest,
    session: Session, 
    query: web::Query<BookingParams>
) -> impl Responder {
    let user_name: Option<String> = session.get("user_name").unwrap_or(None);
    let current_lang = get_lang(&req);
    let accommodation_id = query.accommodation_id.clone().unwrap_or_default();

    let template = PublicBooking1Template {
        user_name,
        current_lang,
        accommodation_id,
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

pub async fn public_booking2(req: HttpRequest, session: Session) -> impl Responder {
    let user_name: Option<String> = session.get("user_name").unwrap_or(None);
    let current_lang = get_lang(&req);

    let template = PublicBooking2Template { 
        user_name, 
        current_lang 
    };
    
    match template.render() {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}