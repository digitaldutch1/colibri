


use actix_session::Session;
use uuid::Uuid;






// Generate csrf token
pub fn generate_csrf_token(
    session: &Session,
) -> String {

    let token =
        Uuid::new_v4().to_string();

    session
        .insert("csrf_token", &token)
        .unwrap();

    token
}



// Verify csrf token
pub fn verify_csrf_token(
    session: &Session,
    form_token: &str,
) -> bool {

    if let Ok(Some(session_token)) =
        session.get::<String>("csrf_token")
    {
        return session_token == form_token;
    }

    false
}