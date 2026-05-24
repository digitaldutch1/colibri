use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use std::env;
use tokio_postgres::NoTls;




// Create customer form
#[derive(Deserialize)]
pub struct CreateCustomerForm {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub postal_code: String,
    pub city: String,
}


// Create customer
pub async fn create_customer(
    form: web::Form<CreateCustomerForm>,
) -> impl Responder {

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

    // Insert customer
    client
        .execute(
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

            VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7
            )
            ",
            &[
                &form.first_name,
                &form.last_name,
                &form.email,
                &form.phone,
                &form.address,
                &form.postal_code,
                &form.city,
            ],
        )
        .await
        .unwrap();

    // Redirect back to customers
    HttpResponse::SeeOther()
        .insert_header((
            actix_web::http::header::LOCATION,
            "/admin/customers",
        ))
        .finish()
}

// Update customer form
#[derive(Deserialize)]
pub struct UpdateCustomerForm {
    pub customer_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub postal_code: String,
    pub city: String,
}

// Update customer
pub async fn update_customer(
    form: web::Form<UpdateCustomerForm>,
) -> impl Responder {

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

    // Update customer
    client
        .execute(
            "
            UPDATE customer

            SET
                first_name = $1,
                last_name = $2,
                email = $3,
                phone = $4,
                address = $5,
                postal_code = $6,
                city = $7

            WHERE id = $8
            ",
            &[
                &form.first_name,
                &form.last_name,
                &form.email,
                &form.phone,
                &form.address,
                &form.postal_code,
                &form.city,
                &form.customer_id,
            ],
        )
        .await
        .unwrap();

    let last_name =
        form.last_name.clone();

    // Redirect back to customers
    HttpResponse::SeeOther()
        .insert_header((
            actix_web::http::header::LOCATION,
            format!(
                "/admin/customers?success=customer_updated&last_name={}",
                last_name
            ),
        ))
        .finish()
}

// Delete customer form
#[derive(Deserialize)]
pub struct DeleteCustomerForm {
    pub customer_id: i32,
}

// Delete customer
pub async fn delete_customer(
    form: web::Form<DeleteCustomerForm>,
) -> impl Responder {

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

    // Get customer lastname
    let row = client
        .query_one(
            "
            SELECT last_name
            FROM customer
            WHERE id = $1
            ",
            &[&form.customer_id],
        )
        .await
        .unwrap();

    let last_name: String =
        row.get(0);

    // Check if customer still has bookings
    let booking_count_row = client
        .query_one(
            "
            SELECT COUNT(*)
            FROM booking
            WHERE customer_id = $1
            ",
            &[&form.customer_id],
        )
        .await
        .unwrap();

    let booking_count: i64 =
        booking_count_row.get(0);

    // Prevent deleting customer with bookings
    if booking_count > 0 {

        return HttpResponse::SeeOther()
            .insert_header((
                actix_web::http::header::LOCATION,
                format!(
                    "/admin/customers?success=customer_delete_error&last_name={}",
                    last_name
                ),
            ))
            .finish();
    }

    // Delete customer
    client
        .execute(
            "
            DELETE FROM customer

            WHERE id = $1
            ",
            &[&form.customer_id],
        )
        .await
        .unwrap();

    // Redirect back to customers
    HttpResponse::SeeOther()
        .insert_header((
            actix_web::http::header::LOCATION,
            format!(
                "/admin/customers?success=customer_deleted&last_name={}",
                last_name
            ),
        ))
        .finish()
}