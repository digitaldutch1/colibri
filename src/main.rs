


mod routes;
mod controllers;
mod templates;

use actix_web::{App, HttpServer};
use actix_files::Files;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .configure(routes::init)
            .service(Files::new("/static", "./src/static"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}