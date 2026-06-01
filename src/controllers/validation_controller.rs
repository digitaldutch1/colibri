
use crate::controllers::booking_controller::*;
use crate::controllers::customer_controller::*;
use crate::controllers::admin_staff_controller::*;


// Public create booking input validation
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

// Admin create booking input validation
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

// Admin customer create input validation
pub fn validate_customer_create(
    form: &CreateCustomerForm
) -> Result<(), String> {

    if form.first_name.trim().is_empty()
        || form.first_name.trim().len() > 50
        || !form.first_name.trim().chars().all(|c|
            c.is_alphabetic() || c == ' ' || c == '-' || c == '\'')
    {
        return Err("error-firstname-invalid".to_string());
    }

    if form.last_name.trim().is_empty()
        || form.last_name.trim().len() > 50
        || !form.last_name.trim().chars().all(|c|
            c.is_alphabetic() || c == ' ' || c == '-' || c == '\'')
    {
        return Err("error-lastname-invalid".to_string());
    }

    if form.address.trim().is_empty()
        || form.address.trim().len() > 100
        || !form.address.trim().chars().all(|c|
            c.is_alphanumeric() || c == ' ')
    {
        return Err("error-address-invalid".to_string());
    }

    if form.postal_code.trim().len() < 4
        || form.postal_code.trim().len() > 10
        || !form.postal_code.trim().chars().all(|c|
            c.is_alphanumeric() || c == ' ')
    {
        return Err("error-zipcode-invalid".to_string());
    }

    if form.city.trim().is_empty()
        || form.city.trim().len() > 50
        || !form.city.trim().chars().all(|c|
            c.is_alphabetic() || c == ' ' || c == '-' || c == '\'')
    {
        return Err("error-city-invalid".to_string());
    }

    if form.phone.trim().len() < 8
        || form.phone.trim().len() > 15
        || !form.phone.trim().chars().all(|c|
            c.is_numeric() || c == '+' || c == ' ' || c == '-')
    {
        return Err("error-phone-invalid".to_string());
    }

    if form.email.trim().is_empty()
        || !form.email.contains('@')
    {
        return Err("error-email-invalid".to_string());
    }

    Ok(())
}

// Admin customer update input validation
pub fn validate_customer_update(
    form: &UpdateCustomerForm
) -> Result<(), String> {

    if form.first_name.trim().is_empty()
        || form.first_name.trim().len() > 50
        || !form.first_name.trim().chars().all(|c|
            c.is_alphabetic() || c == ' ' || c == '-' || c == '\'')
    {
        return Err("error-firstname-invalid".to_string());
    }

    if form.last_name.trim().is_empty()
        || form.last_name.trim().len() > 50
        || !form.last_name.trim().chars().all(|c|
            c.is_alphabetic() || c == ' ' || c == '-' || c == '\'')
    {
        return Err("error-lastname-invalid".to_string());
    }

    if form.address.trim().is_empty()
        || form.address.trim().len() > 100
        || !form.address.trim().chars().all(|c|
            c.is_alphanumeric() || c == ' ')
    {
        return Err("error-address-invalid".to_string());
    }

    if form.postal_code.trim().len() < 4
        || form.postal_code.trim().len() > 10
        || !form.postal_code.trim().chars().all(|c|
            c.is_alphanumeric() || c == ' ')
    {
        return Err("error-zipcode-invalid".to_string());
    }

    if form.city.trim().is_empty()
        || form.city.trim().len() > 50
        || !form.city.trim().chars().all(|c|
            c.is_alphabetic() || c == ' ' || c == '-' || c == '\'')
    {
        return Err("error-city-invalid".to_string());
    }

    if form.phone.trim().len() < 8
        || form.phone.trim().len() > 15
        || !form.phone.trim().chars().all(|c|
            c.is_numeric() || c == '+' || c == ' ' || c == '-')
    {
        return Err("error-phone-invalid".to_string());
    }

    if form.email.trim().is_empty()
        || !form.email.contains('@')
    {
        return Err("error-email-invalid".to_string());
    }

    Ok(())
}

// Admin staff create input validation
pub fn validate_staff_create(
    form: &CreateStaffForm
) -> Result<(), String> {

    if form.first_name.trim().is_empty()
        || form.first_name.trim().len() > 50
        || !form.first_name.trim().chars().all(|c|
            c.is_alphabetic() || c == ' ' || c == '-' || c == '\'')
    {
        return Err("error-firstname-invalid".to_string());
    }

    if form.last_name.trim().is_empty()
        || form.last_name.trim().len() > 50
        || !form.last_name.trim().chars().all(|c|
            c.is_alphabetic() || c == ' ' || c == '-' || c == '\'')
    {
        return Err("error-lastname-invalid".to_string());
    }

    if form.email.trim().is_empty()
        || !form.email.contains('@')
    {
        return Err("error-email-invalid".to_string());
    }

    let password = &form.password;

    if password.len() < 8
        || !password.chars().any(|c| c.is_lowercase())
        || !password.chars().any(|c| c.is_uppercase())
        || !password.chars().any(|c| c.is_numeric())
        || !password.chars().any(|c| !c.is_alphanumeric())
    {
        return Err("error-password-invalid".to_string());
    }

    Ok(())
}

// Admin staff update input validation
pub fn validate_staff_update(
    form: &UpdateStaffForm
) -> Result<(), String> {

    if form.first_name.trim().is_empty()
        || form.first_name.trim().len() > 50
        || !form.first_name.trim().chars().all(|c|
            c.is_alphabetic() || c == ' ' || c == '-' || c == '\'')
    {
        return Err("error-firstname-invalid".to_string());
    }

    if form.last_name.trim().is_empty()
        || form.last_name.trim().len() > 50
        || !form.last_name.trim().chars().all(|c|
            c.is_alphabetic() || c == ' ' || c == '-' || c == '\'')
    {
        return Err("error-lastname-invalid".to_string());
    }

    if form.email.trim().is_empty()
        || !form.email.contains('@')
    {
        return Err("error-email-invalid".to_string());
    }

    if !form.password.trim().is_empty() {

        let password = &form.password;

        if password.len() < 8
            || !password.chars().any(|c| c.is_lowercase())
            || !password.chars().any(|c| c.is_uppercase())
            || !password.chars().any(|c| c.is_numeric())
            || !password.chars().any(|c| !c.is_alphanumeric())
        {
            return Err("error-password-invalid".to_string());
        }
    }

    Ok(())
}