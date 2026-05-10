


mod routes;
mod controllers;
mod templates;
mod db;

use actix_web::{App, HttpServer};
use actix_files::Files;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load the .env file
    dotenv().ok();

    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .configure(routes::init)
            .service(Files::new("/static", "./src/static"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}