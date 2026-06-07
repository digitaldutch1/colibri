use colibri::controllers::booking_controller::PublicBookingForm;
use colibri::controllers::validation_controller::*;





// Validation tests
fn valid_public_booking_form() -> PublicBookingForm {

    PublicBookingForm {
        csrf_token: String::new(),
        booking_id: "1".to_string(),
        lock_token: "token".to_string(),
        accommodation_id: "1".to_string(),
        check_in_date: "2026-07-01".to_string(),
        check_out_date: "2026-07-03".to_string(),
        first_name: "Dennis".to_string(),
        last_name: "Hettinga".to_string(),
        address: "Teststraat 1".to_string(),
        zip_code: "1234AB".to_string(),
        city: "Tilburg".to_string(),
        phone: "0612345678".to_string(),
        email: "test@test.nl".to_string(),
        tos_accepted: Some("yes".to_string()),
    }
}



#[test]
fn ut_01_valid_email() {
    let form = valid_public_booking_form();
    assert!(validate_public_booking(&form).is_ok());
}

#[test]
fn ut_02_invalid_email() {
    let mut form = valid_public_booking_form();
    form.email = "ongeldig-email".to_string();
    assert!(validate_public_booking(&form).is_err());
}

#[test]
fn ut_03_valid_firstname() {
    let form = valid_public_booking_form();
    assert!(validate_public_booking(&form).is_ok());
}

#[test]
fn ut_04_empty_firstname() {
    let mut form = valid_public_booking_form();
    form.first_name = "".to_string();
    assert!(validate_public_booking(&form).is_err());
}

#[test]
fn ut_05_valid_phone() {
    let form = valid_public_booking_form();
    assert!(validate_public_booking(&form).is_ok());
}

#[test]
fn ut_06_invalid_phone() {
    let mut form = valid_public_booking_form();
    form.phone = "06-12345678".to_string();
    assert!(validate_public_booking(&form).is_err());
}

#[test]
fn ut_07_valid_zipcode() {
    let form = valid_public_booking_form();
    assert!(validate_public_booking(&form).is_ok());
}

#[test]
fn ut_08_invalid_zipcode() {
    let mut form = valid_public_booking_form();
    form.zip_code = "!!!!".to_string();
    assert!(validate_public_booking(&form).is_err());
}

#[test]
fn ut_09_valid_lastname() {
    let form = valid_public_booking_form();
    assert!(validate_public_booking(&form).is_ok());
}

#[test]
fn ut_10_empty_lastname() {
    let mut form = valid_public_booking_form();
    form.last_name = "".to_string();
    assert!(validate_public_booking(&form).is_err());
}

#[test]
fn ut_11_valid_address() {
    let form = valid_public_booking_form();
    assert!(validate_public_booking(&form).is_ok());
}

#[test]
fn ut_12_empty_address() {
    let mut form = valid_public_booking_form();
    form.address = "".to_string();
    assert!(validate_public_booking(&form).is_err());
}

#[test]
fn ut_13_valid_city() {
    let form = valid_public_booking_form();
    assert!(validate_public_booking(&form).is_ok());
}

#[test]
fn ut_14_empty_city() {
    let mut form = valid_public_booking_form();
    form.city = "".to_string();
    assert!(validate_public_booking(&form).is_err());
}


