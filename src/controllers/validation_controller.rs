
use crate::controllers::booking_controller::PublicBookingForm;
use crate::controllers::booking_controller::AdminBookingForm;


pub fn validate_public_booking(form: &PublicBookingForm) -> Result<(), String> {
    
    // First name: min 1, max 50, letters, spaces, apostrophes and hyphens
    if form.first_name.trim().is_empty()
        || form.first_name.trim().len() > 50
        || !form.first_name.trim().chars().all(|c|
            c.is_alphabetic() || c == ' ' || c == '-' || c == '\'')
    {
        return Err("error-firstname-invalid".to_string());
    }

    // Last name: min 1, max 50, letters, spaces, apostrophes and hyphens
    if form.last_name.trim().is_empty()
        || form.last_name.trim().len() > 50
        || !form.last_name.trim().chars().all(|c|
            c.is_alphabetic() || c == ' ' || c == '-' || c == '\'')
    {
        return Err("error-lastname-invalid".to_string());
    }

    // Address: min 1, max 100, letters, numbers, spaces only (no special characters like , or .)
    if form.address.trim().is_empty() || form.address.trim().len() > 100 || 
    !form.address.trim().chars().all(|c| c.is_alphanumeric() || c == ' ') {
        return Err("error-address-invalid".to_string());
    }
    
    // Zip code: min 4, max 10, letters, numbers and spaces
    if form.zip_code.trim().len() < 4
        || form.zip_code.trim().len() > 10
        || !form.zip_code.trim().chars().all(|c|
            c.is_alphanumeric() || c == ' ')
    {
        return Err("error-zipcode-invalid".to_string());
    }

    // City: min 1, max 50, letters, spaces, apostrophes and hyphens
    if form.city.trim().is_empty()
        || form.city.trim().len() > 50
        || !form.city.trim().chars().all(|c|
            c.is_alphabetic() || c == ' ' || c == '-' || c == '\'')
    {
        return Err("error-city-invalid".to_string());
    }

    // Phone: min 8, max 15, numbers, +, spaces and hyphens
    if form.phone.trim().len() < 8
        || form.phone.trim().len() > 15
        || !form.phone.trim().chars().all(|c|
            c.is_numeric() || c == '+' || c == ' ' || c == '-')
    {
        return Err("error-phone-invalid".to_string());
    }

    // Email: must contain @
    if form.email.trim().is_empty() || !form.email.contains('@') {
        return Err("error-email-invalid".to_string());
    }

    Ok(())
}

pub fn validate_admin_booking(form: &AdminBookingForm) -> Result<(), String> {

    // First name: min 1, max 50, letters, spaces, apostrophes and hyphens
    if form.first_name.trim().is_empty()
        || form.first_name.trim().len() > 50
        || !form.first_name.trim().chars().all(|c|
            c.is_alphabetic() || c == ' ' || c == '-' || c == '\'')
    {
        return Err("error-firstname-invalid".to_string());
    }

    // Last name: min 1, max 50, letters, spaces, apostrophes and hyphens
    if form.last_name.trim().is_empty()
        || form.last_name.trim().len() > 50
        || !form.last_name.trim().chars().all(|c|
            c.is_alphabetic() || c == ' ' || c == '-' || c == '\'')
    {
        return Err("error-lastname-invalid".to_string());
    }

    // Address: min 1, max 100, letters, numbers and spaces only
    if form.address.trim().is_empty()
        || form.address.trim().len() > 100
        || !form.address.trim().chars().all(|c|
            c.is_alphanumeric() || c == ' ')
    {
        return Err("error-address-invalid".to_string());
    }

    // Zip code: min 4, max 10, letters, numbers and spaces
    if form.zip_code.trim().len() < 4
        || form.zip_code.trim().len() > 10
        || !form.zip_code.trim().chars().all(|c|
            c.is_alphanumeric() || c == ' ')
    {
        return Err("error-zipcode-invalid".to_string());
    }

    // City: min 1, max 50, letters, spaces, apostrophes and hyphens
    if form.city.trim().is_empty()
        || form.city.trim().len() > 50
        || !form.city.trim().chars().all(|c|
            c.is_alphabetic() || c == ' ' || c == '-' || c == '\'')
    {
        return Err("error-city-invalid".to_string());
    }

    // Phone: min 8, max 15, numbers, +, spaces and hyphens
    if form.phone.trim().len() < 8
        || form.phone.trim().len() > 15
        || !form.phone.trim().chars().all(|c|
            c.is_numeric() || c == '+' || c == ' ' || c == '-')
    {
        return Err("error-phone-invalid".to_string());
    }

    // Email: must contain @
    if form.email.trim().is_empty() || !form.email.contains('@') {
        return Err("error-email-invalid".to_string());
    }

    Ok(())
}