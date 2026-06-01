


// Imports required libraries for web handling, sessions, database, and time
use actix_web::{web, HttpResponse, Responder, HttpRequest};
use actix_session::Session;
use serde::Deserialize;
use tokio_postgres::NoTls;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use bcrypt::verify;





// login form struct
#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}



// Returns current time as Unix timestamp
// Gets current time to store login time and check session timeout
fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}



// Admin login: checks user credentials and creates session
pub async fn login_admin(
    form: web::Form<LoginForm>,
    session: Session,
) -> impl Responder {

    let now = now_unix();

    let mut attempts = session.get::<i32>("login_attempts").unwrap_or(None).unwrap_or(0);
    let last_attempt = session.get::<i64>("last_attempt").unwrap_or(None).unwrap_or(0);

    let max_attempts = 3;
    let block_time = 300; // seconds

    if now - last_attempt >= block_time {
        attempts = 0;
    }

    if attempts >= max_attempts && now - last_attempt < block_time {

        let remaining = block_time - (now - last_attempt);
        let minutes = (remaining + 59) / 60;

        let time_text = if minutes == 1 {
            "1 minuut".to_string()
        } else {
            format!("{} minuten", minutes)
        };

        let redirect_url = format!(
            "/admin/login?error=Teveel pogingen, probeer over {} opnieuw",
            time_text
        );

        return HttpResponse::Found()
            .append_header(("Location", redirect_url))
            .finish();
    }

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let (client, connection) =
        tokio_postgres::connect(&database_url, NoTls)
            .await
            .expect("DB connect failed");

    actix_web::rt::spawn(async move {
        let _ = connection.await;
    });

    let row = client
        .query_opt(
            "SELECT first_name, password_hash, role FROM \"user\" WHERE email = $1",
            &[&form.email],
        )
        .await
        .expect("Query failed");

    if let Some(row) = row {
        let first_name: String = row.get(0);
        let db_password_hash: String = row.get(1);
        let role: String = row.get(2);

        if verify(&form.password, &db_password_hash).unwrap_or(false) {

            session.insert("login_attempts", 0).unwrap();
            session.insert("last_attempt", now).unwrap();
            session.insert("logged_in", true).unwrap();
            session.insert("user_name", first_name).unwrap();
            session.insert("user_role", role).unwrap();
            session.insert("login_at", now).unwrap();
            session.insert("last_seen", now).unwrap();

            return HttpResponse::Found()
                .append_header(("Location", "/admin"))
                .finish();
        }
    }

    attempts += 1;
    session.insert("login_attempts", attempts).unwrap();
    session.insert("last_attempt", now).unwrap();

    HttpResponse::Found()
        .append_header(("Location", "/admin/login?error=Email en wachtwoord komen niet overeen"))
        .finish()
}



// Admin logout: clears session and redirects user
pub async fn logout_admin(
    req: HttpRequest,
    session: Session,
) -> impl Responder {

    // 1. Remove all session data
    session.purge();

    // 2. Read optional redirect query parameter
    let redirect_public =
        req.query_string().contains("redirect=public");

    // 3. Determine redirect location
    let redirect_to = if redirect_public {
        "/"
    } else {
        "/admin/login"
    };

    // 4. Redirect user
    HttpResponse::Found()
        .append_header(("Location", redirect_to))
        .finish()
}