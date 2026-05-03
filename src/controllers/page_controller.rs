


use actix_web::{web, HttpResponse, Responder};
use actix_session::Session;
use askama::Template;
use crate::templates::*;
use serde::Deserialize;



pub async fn public_home(session: Session) -> impl Responder {
    let user_name: Option<String> = session.get("user_name").unwrap_or(None);
    let template = HomePublicTemplate { user_name };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

pub async fn contact_page(session: Session) -> impl Responder {
    let user_name: Option<String> = session.get("user_name").unwrap_or(None);

    let template = ContactTemplate {
        user_name,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

pub async fn tos_page(session: Session) -> impl Responder {
    let user_name: Option<String> = session.get("user_name").unwrap_or(None);

    let template = TosTemplate  {
        user_name,
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
    session: Session, 
    query: web::Query<BookingParams>
) -> impl Responder {
    let user_name: Option<String> = session.get("user_name").unwrap_or(None);
    let accommodation_id = query.accommodation_id.clone().unwrap_or_default();

    let template = PublicBooking1Template {
        user_name,
        accommodation_id,
    };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

pub async fn public_booking2(session: Session) -> impl Responder {
    let user_name: Option<String> = session.get("user_name").unwrap_or(None);
    let template = PublicBooking2Template { user_name };
    
    match template.render() {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}