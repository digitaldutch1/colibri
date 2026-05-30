use lettre::transport::smtp::authentication::Credentials;
use lettre::{message::header, Message, SmtpTransport, Transport};
use std::env;



// Gmail / Mailpit switch based on MAIL_DRIVER setting in .env
fn create_mailer(smtp_user: &str, smtp_password: &str) -> Result<SmtpTransport, String> {
    let mail_driver = env::var("MAIL_DRIVER").unwrap_or_else(|_| "gmail".to_string());

    if mail_driver == "mailpit" {
        return Ok(SmtpTransport::builder_dangerous("127.0.0.1")
            .port(1025)
            .build());
    }

    let creds = Credentials::new(smtp_user.to_string(), smtp_password.to_string());

    Ok(SmtpTransport::starttls_relay("smtp.gmail.com")
        .map_err(|e| e.to_string())?
        .credentials(creds)
        .port(587)
        .build())
}


// Booking confirmation email
pub async fn send_confirmation_email(
    to_email: &str,
    first_name: &str,
    last_name: &str,
    phone: &str,
    address: &str,
    zip_code: &str,
    city: &str,
    accommodation_name: &str,
    check_in: &str,
    check_out: &str,
    payment_token: &str,
    cancel_token: &str,
    nights: i64,
    price_per_night: f64,
    total_price: f64,
) -> Result<(), String> {
    let smtp_user = env::var("SMTP_USER").map_err(|_| "SMTP_USER missing".to_string())?;
    let smtp_password = env::var("SMTP_PASSWORD").map_err(|_| "SMTP_PASSWORD missing".to_string())?;
    let app_url = env::var("APP_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());

    let payment_link = format!("{}/payment/{}/confirm", app_url, payment_token);
    let cancel_link = format!("{}/cancel/{}", app_url, cancel_token);

    let email_body = format!(
        "<html>
        <body style='font-family: Arial, sans-serif;'>
            <p>Dear {},</p>

            <p>Thank you for your booking at Camping de Colibri.</p>

            <h3>Your booking overview:</h3>

            <p>
                {} {}<br>
                {}<br>
                {} {}<br>
                Phone: {}
            </p>

            <p>You booked a <strong>{}</strong>.</p>

            <p>
                From <strong>{}</strong> until <strong>{}</strong><br>
                <small>({} nights at €{:.2} per night)</small>
            </p>

            <p><strong>Total amount: €{:.2}</strong></p>

            <p>
                <strong>Payment:</strong><br>
                <a href='{}'>Click here to pay</a>
            </p>

            <p>
                <strong>Cancel booking:</strong><br>
                <a href='{}'>Click here to cancel</a>
            </p>

            <p>
                Kind regards,<br>
                Team Colibri
            </p>
        </body>
        </html>",
        first_name,
        first_name,
        last_name,
        address,
        zip_code,
        city,
        phone,
        accommodation_name,
        check_in,
        check_out,
        nights,
        price_per_night,
        total_price,
        payment_link,
        cancel_link
    );

    let email = Message::builder()
        .from(format!("Colibri <{}>", smtp_user).parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject("Booking confirmation")
        .header(header::ContentType::TEXT_HTML)
        .body(email_body)
        .map_err(|e| e.to_string())?;

    let mailer = create_mailer(&smtp_user, &smtp_password)?;

    mailer.send(&email).map_err(|e| e.to_string())?;

    Ok(())
}



// Payment confirmation email
pub async fn send_invoice_email(
    to_email: &str,
    first_name: &str,
    invoice_number: &str,
    total_price: &str,
) -> Result<(), String> {
    let smtp_user = env::var("SMTP_USER").map_err(|_| "SMTP_USER missing".to_string())?;
    let smtp_password = env::var("SMTP_PASSWORD").map_err(|_| "SMTP_PASSWORD missing".to_string())?;

    let email_body = format!(
        "<html>
        <body style='font-family: Arial, sans-serif;'>
            <p>Dear {},</p>

            <p>Your payment has been successfully received.</p>

            <p>
                Invoice number: <strong>{}</strong><br>
                Total amount: <strong>€{}</strong>
            </p>

            <p>Thank you for your booking at Camping de Colibri.</p>

            <p>
                Kind regards,<br>
                Team Colibri
            </p>
        </body>
        </html>",
        first_name,
        invoice_number,
        total_price
    );

    let email = Message::builder()
        .from(format!("Colibri <{}>", smtp_user).parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject("Payment confirmation")
        .header(header::ContentType::TEXT_HTML)
        .body(email_body)
        .map_err(|e| e.to_string())?;

    let mailer = create_mailer(&smtp_user, &smtp_password)?;

    mailer.send(&email).map_err(|e| e.to_string())?;

    Ok(())
}



// Cancel confirmation email
pub async fn send_cancel_confirmation_email(
    to_email: &str,
    first_name: &str,
    invoice_number: &str,
) -> Result<(), String> {
    let smtp_user = env::var("SMTP_USER").map_err(|_| "SMTP_USER missing".to_string())?;
    let smtp_password = env::var("SMTP_PASSWORD").map_err(|_| "SMTP_PASSWORD missing".to_string())?;

    let email_body = format!(
        "<html>
        <body style='font-family: Arial, sans-serif;'>
            <p>Dear {},</p>

            <p>Your booking at Camping de Colibri has been successfully cancelled.</p>

            <p>Invoice number: <strong>{}</strong></p>

            <p>We hope to welcome you another time.</p>

            <p>
                Kind regards,<br>
                Team Colibri
            </p>
        </body>
        </html>",
        first_name,
        invoice_number
    );

    let email = Message::builder()
        .from(format!("Colibri <{}>", smtp_user).parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject("Booking cancellation")
        .header(header::ContentType::TEXT_HTML)
        .body(email_body)
        .map_err(|e| e.to_string())?;

    let mailer = create_mailer(&smtp_user, &smtp_password)?;

    mailer.send(&email).map_err(|e| e.to_string())?;
    Ok(())
}

// Booking expired email
pub async fn send_booking_expired_email(
    to_email: &str,
    first_name: &str,
) -> Result<(), String> {

    let smtp_user =
        env::var("SMTP_USER")
            .map_err(|_| "SMTP_USER missing".to_string())?;

    let smtp_password =
        env::var("SMTP_PASSWORD")
            .map_err(|_| "SMTP_PASSWORD missing".to_string())?;

    let email_body = format!(
        "<html>
        <body style='font-family: Arial, sans-serif;'>

            <p>Dear {},</p>

            <p>
                Your booking has expired because payment was not received within 7 days.
            </p>

            <p>
                The reservation has been cancelled automatically and the accommodation has been released.
            </p>

            <p>
                Kind regards,<br>
                Team Colibri
            </p>

        </body>
        </html>",
        first_name
    );

    let email = Message::builder()
        .from(format!("Colibri <{}>", smtp_user).parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject("Booking expired")
        .header(header::ContentType::TEXT_HTML)
        .body(email_body)
        .map_err(|error| error.to_string())?;

    let mailer =
        create_mailer(&smtp_user, &smtp_password)?;

    mailer.send(&email)
        .map_err(|error| error.to_string())?;

    Ok(())
}

// Booking update confirmation email
pub async fn send_booking_update_email(
    to_email: &str,
    first_name: &str,
    accommodation_name: &str,
    check_in: &str,
    check_out: &str,
    nights: i64,
    price_per_night: f64,
    total_price: f64,
) -> Result<(), String> {

    let smtp_user =
        env::var("SMTP_USER")
            .map_err(|_| "SMTP_USER missing".to_string())?;

    let smtp_password =
        env::var("SMTP_PASSWORD")
            .map_err(|_| "SMTP_PASSWORD missing".to_string())?;

    let email_body = format!(
        "<html>
        <body style='font-family: Arial, sans-serif;'>

            <p>Dear {},</p>

            <p>
                Your booking at Camping de Colibri
                has been updated.
            </p>

            <h3>Updated booking overview:</h3>

            <p>
                Accommodation:
                <strong>{}</strong>
            </p>

            <p>
                From <strong>{}</strong>
                until <strong>{}</strong><br>

                <small>
                    ({} nights at €{:.2} per night)
                </small>
            </p>

            <p>
                <strong>
                    Total amount: €{:.2}
                </strong>
            </p>

            <p>
                Kind regards,<br>
                Team Colibri
            </p>

        </body>
        </html>",
        first_name,
        accommodation_name,
        check_in,
        check_out,
        nights,
        price_per_night,
        total_price
    );

    let email = Message::builder()
        .from(format!("Colibri <{}>", smtp_user).parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject("Booking updated")
        .header(header::ContentType::TEXT_HTML)
        .body(email_body)
        .map_err(|e| e.to_string())?;

    let mailer =
        create_mailer(&smtp_user, &smtp_password)?;

    mailer.send(&email)
        .map_err(|e| e.to_string())?;

    Ok(())
}

// Booking status update email
pub async fn send_booking_status_email(
    to_email: &str,
    first_name: &str,
    status: &str,
    invoice_number: &str,
) -> Result<(), String> {

    let smtp_user =
        env::var("SMTP_USER")
            .map_err(|_| "SMTP_USER missing".to_string())?;

    let smtp_password =
        env::var("SMTP_PASSWORD")
            .map_err(|_| "SMTP_PASSWORD missing".to_string())?;

    let email_body = format!(
        "<html>
        <body style='font-family: Arial, sans-serif;'>

            <p>Dear {},</p>

            <p>
                The status of your booking at Camping de Colibri
                has been updated.
            </p>

            <p>
                New booking status:
                <strong>{}</strong>
            </p>

            {}

            <p>
                Kind regards,<br>
                Team Colibri
            </p>

        </body>
        </html>",
        first_name,
        status,

        if invoice_number.is_empty() {

            "".to_string()

        } else {

            format!(
                "<p>
                    Invoice number:
                    <strong>{}</strong>
                </p>",
                invoice_number
            )
        }
    );

    let email = Message::builder()
        .from(format!("Colibri <{}>", smtp_user).parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject("Booking status updated")
        .header(header::ContentType::TEXT_HTML)
        .body(email_body)
        .map_err(|e| e.to_string())?;

    let mailer =
        create_mailer(&smtp_user, &smtp_password)?;

    mailer.send(&email)
        .map_err(|e| e.to_string())?;

    Ok(())
}