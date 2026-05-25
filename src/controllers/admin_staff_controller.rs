use serde::Deserialize;
use actix_web::{web, HttpResponse, Responder};
use actix_session::Session;
use std::env;
use bcrypt::{hash, DEFAULT_COST};
use tokio_postgres::NoTls;


// Admin staff create struct
#[derive(Deserialize)]
pub struct CreateStaffForm {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

// Admin staff create
pub async fn create_staff(
    session: Session,
    form: web::Form<CreateStaffForm>,
) -> impl Responder {

    // Check admin role
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

    // Hash password
    let password_hash =
        hash(&form.password, DEFAULT_COST)
            .unwrap();

    // Create staff account
    client
        .execute(
            "
            INSERT INTO \"user\" (
                first_name,
                last_name,
                email,
                password_hash,
                role
            )

            VALUES (
                $1,
                $2,
                $3,
                $4,
                'staff'
            )
            ",
            &[
                &form.first_name,
                &form.last_name,
                &form.email,
                &password_hash,
            ],
        )
        .await
        .unwrap();

    let last_name =
        form.last_name.clone();

    // Redirect back to staff
    HttpResponse::SeeOther()
        .insert_header((
            actix_web::http::header::LOCATION,
            format!(
                "/admin/staff?success=staff_created&last_name={}",
                last_name
            ),
        ))
        .finish()
}

// Admin staff update struct
#[derive(Deserialize)]
pub struct UpdateStaffForm {
    pub user_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

// Admin staff update
pub async fn update_staff(
    session: Session,
    form: web::Form<UpdateStaffForm>,
) -> impl Responder {

    // Check admin role
    let user_role: String =
        session.get::<String>("user_role")
            .unwrap_or(None)
            .unwrap_or_default();

    if user_role != "admin" {

        return HttpResponse::Found()
            .append_header(("Location", "/admin/staff"))
            .finish();
    }

    // Prevent changing primary admin role
    if form.user_id == 1 && form.role != "admin" {

        return HttpResponse::Found()
            .append_header((
                "Location",
                "/admin/staff?success=primary_admin_role_error"
            ))
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

    // Update without password change
    if form.password.trim().is_empty() {

        client
            .execute(
                "
                UPDATE \"user\"

                SET
                    first_name = $1,
                    last_name = $2,
                    email = $3,
                    role = $4

                WHERE id = $5
                ",
                &[
                    &form.first_name,
                    &form.last_name,
                    &form.email,
                    &form.role,
                    &form.user_id,
                ],
            )
            .await
            .unwrap();

    } else {

        // Hash new password
        let password_hash =
            hash(&form.password, DEFAULT_COST)
                .unwrap();

        // Update with password
        client
            .execute(
                "
                UPDATE \"user\"

                SET
                    first_name = $1,
                    last_name = $2,
                    email = $3,
                    password_hash = $4,
                    role = $5

                WHERE id = $6
                ",
                &[
                    &form.first_name,
                    &form.last_name,
                    &form.email,
                    &password_hash,
                    &form.role,
                    &form.user_id,
                ],
            )
            .await
            .unwrap();
    }

    let last_name =
        form.last_name.clone();

    // Redirect back to staff
    HttpResponse::SeeOther()
        .insert_header((
            actix_web::http::header::LOCATION,
            format!(
                "/admin/staff?success=staff_updated&last_name={}",
                last_name
            ),
        ))
        .finish()
}

// Admin staff delete struct
#[derive(Deserialize)]
pub struct DeleteStaffForm {
    pub user_id: i32,
}

// Admin staff delete
pub async fn delete_staff(
    session: Session,
    form: web::Form<DeleteStaffForm>,
) -> impl Responder {

    // Check admin role
    let user_role: String =
        session.get::<String>("user_role")
            .unwrap_or(None)
            .unwrap_or_default();

    if user_role != "admin" {

        return HttpResponse::Found()
            .append_header(("Location", "/admin/staff"))
            .finish();
    }

    // Prevent deleting primary admin
    if form.user_id == 1 {

        return HttpResponse::Found()
            .append_header((
                "Location",
                "/admin/staff?success=primary_admin_delete_error"
            ))
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

    // Get lastname
    let row = client
        .query_one(
            "
            SELECT last_name
            FROM \"user\"
            WHERE id = $1
            ",
            &[&form.user_id],
        )
        .await
        .unwrap();

    let last_name: String =
        row.get(0);

    // Delete user
    client
        .execute(
            "
            DELETE FROM \"user\"

            WHERE id = $1
            ",
            &[&form.user_id],
        )
        .await
        .unwrap();

    // Redirect back
    HttpResponse::SeeOther()
        .insert_header((
            actix_web::http::header::LOCATION,
            format!(
                "/admin/staff?success=staff_deleted&last_name={}",
                last_name
            ),
        ))
        .finish()
}