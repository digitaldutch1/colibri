


mod routes;
mod controllers;
mod templates;
mod db;

use actix_web::{App, HttpServer};
use actix_files::Files;
use dotenv::dotenv;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Load the .env file
    dotenv().ok();

    // Session secret key
    let secret_key = Key::generate();

    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()

            // Session middleware
            .wrap(
                SessionMiddleware::new(
                    CookieSessionStore::default(),
                    secret_key.clone(),
                )
            )

            // Routes
            .configure(routes::init)

            // Static files
            .service(Files::new("/static", "./src/static"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}