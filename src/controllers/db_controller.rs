


use std::env;
use tokio_postgres::NoTls;
use crate::db::Accommodation;

pub async fn get_all_accommodations() -> Vec<Accommodation> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect manually like in your backup
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .expect("Failed to connect");

    // Spawn the connection in the background
    actix_web::rt::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client
        .query("SELECT id, name, total_units, created_at FROM accommodation", &[])
        .await
        .expect("Query failed");

    let mut accommodations = Vec::new();
    for row in rows {
        accommodations.push(Accommodation {
            id: row.get(0),
            name: row.get(1),
            total_units: row.get(2),
            created_at: row.get(3),
        });
    }
    accommodations
}