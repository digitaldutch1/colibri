


use askama::Template;
use unic_langid::LanguageIdentifier;
use crate::db::CustomerRow;




// Public
// Fluent language switch
// Load locales from the root /locales folder
fluent_templates::static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en",
    };
}

// Helper trait to make i18n work in Askama templates
pub trait I18nTemplate {
    fn lang(&self) -> &str;
    fn i18n(&self, key: &str) -> String {
        use fluent_templates::Loader;
        let lang_id: LanguageIdentifier = self.lang().parse().unwrap_or_else(|_| "en".parse().unwrap());
        LOCALES.lookup(&lang_id, key).unwrap_or_else(|| key.to_string())
    }
}



// Public page template structs and Trait implementations EN/NL
#[derive(Template)]
#[template(path = "pages/public_home.html")]
pub struct HomePublicTemplate {
    pub user_name: Option<String>,
    pub current_lang: String,
    pub payment: String,
    pub invoice: String,
    pub error: String,
}

impl I18nTemplate for HomePublicTemplate {
    fn lang(&self) -> &str { &self.current_lang }
}



#[derive(Template)]
#[template(path = "pages/public_contact.html")]
pub struct ContactTemplate {
    pub user_name: Option<String>,
    pub current_lang: String,
}

impl I18nTemplate for ContactTemplate {
    fn lang(&self) -> &str { &self.current_lang }
}



#[derive(Template)]
#[template(path = "pages/public_tos.html")]
pub struct TosTemplate {
    pub user_name: Option<String>,
    pub current_lang: String,
}

impl I18nTemplate for TosTemplate {
    fn lang(&self) -> &str { &self.current_lang }
}



// Public booking template structs and Trait implementations EN/NL
#[derive(Template)]
#[template(path = "pages/public_booking1.html")]
pub struct PublicBooking1Template {
    pub user_name: Option<String>,
    pub accommodation_id: String,
    pub current_lang: String,
    pub error: String,
}

impl I18nTemplate for PublicBooking1Template {
    fn lang(&self) -> &str { &self.current_lang }
}



#[derive(Template)]
#[template(path = "pages/public_booking2.html")]
pub struct PublicBooking2Template {
    pub user_name: Option<String>,
    pub current_lang: String,
    pub booking_id: String,
    pub lock_token: String,
    pub accommodation_id: String,
    pub check_in_date: String,
    pub check_out_date: String,
}

impl I18nTemplate for PublicBooking2Template {
    fn lang(&self) -> &str { &self.current_lang }
}



#[derive(Template)]
#[template(path = "pages/public_booking_overview.html")]
pub struct PublicBookingOverviewTemplate {
    pub user_name: Option<String>,
    pub current_lang: String,
    pub success: bool,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub zip_code: String,
    pub city: String,
    pub phone: String,
    pub accommodation: String,
    pub check_in: String,
    pub check_out: String,
    pub nights: i64,
    pub price_per_night: String,
    pub total_price: String,
    pub payment_token: String,
}

impl I18nTemplate for PublicBookingOverviewTemplate {
    fn lang(&self) -> &str {
        &self.current_lang
    }
}





//Admin
// Admin login page template struct and trait implemantation EN/NL
#[derive(Template)]
#[template(path = "pages/admin_login.html")]
pub struct AdminLoginTemplate {
    pub user_name: Option<String>,
    pub current_lang: String,
    pub error: Option<String>,
}

impl I18nTemplate for AdminLoginTemplate {
    fn lang(&self) -> &str { &self.current_lang }
}

// Admin page template structs and Trait implementations EN/NL
#[derive(Template)]
#[template(path = "pages/admin_home.html")]
pub struct AdminHomeTemplate {
    pub user_name: Option<String>,
    pub current_lang: String,
}

impl I18nTemplate for AdminHomeTemplate {
    fn lang(&self) -> &str { &self.current_lang }
}

// Admin booking read row
// Admin booking read row
pub struct BookingRow {
    pub id: i32,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub unit_code: String,
    pub invoice_number: Option<String>,
    pub check_in: String,
    pub check_out: String,
    pub status: String,
    pub source: Option<String>,
}

// Admin bookings read template
#[derive(Template)]
#[template(path = "pages/admin_bookings_read.html")]
pub struct AdminBookingsReadTemplate {
    pub user_name: Option<String>,
    pub current_lang: String,
    pub bookings: Vec<BookingRow>,
    pub user_role: String,
    pub error: Option<String>,
    pub success: String,
}

impl I18nTemplate for AdminBookingsReadTemplate {
    fn lang(&self) -> &str { &self.current_lang }
}

// Admin booking step 1 create template
#[derive(Template)]
#[template(path = "pages/admin_booking1_create.html")]
pub struct AdminBooking1CreateTemplate {
    pub user_name: Option<String>,
    pub current_lang: String,
    pub accommodation_id: String,
    pub error: String,
}

impl I18nTemplate for AdminBooking1CreateTemplate {
    fn lang(&self) -> &str {
        &self.current_lang
    }
}


// Admin booking step 2 create template
#[derive(Template)]
#[template(path = "pages/admin_booking2_create.html")]
pub struct AdminBooking2CreateTemplate {
    pub user_name: Option<String>,
    pub current_lang: String,
    pub accommodation_id: String,
    pub check_in_date: String,
    pub check_out_date: String,
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub zip_code: String,
    pub city: String,
    pub phone: String,
    pub email: String,
    pub error: Option<String>,
}

impl I18nTemplate for AdminBooking2CreateTemplate {
    fn lang(&self) -> &str {
        &self.current_lang
    }
}

// Admin booking overview page template struct and trait implementation
#[derive(Template)]
#[template(path = "pages/admin_booking_overview.html")]
pub struct AdminBookingOverviewTemplate {
    pub user_name: Option<String>,
    pub current_lang: String,
    pub success: bool,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub zip_code: String,
    pub city: String,
    pub phone: String,
    pub accommodation: String,
    pub check_in: String,
    pub check_out: String,
    pub nights: i64,
    pub price_per_night: String,
    pub total_price: String,
}

impl I18nTemplate for AdminBookingOverviewTemplate {
    fn lang(&self) -> &str {
        &self.current_lang
    }
}

// Admin booking update template
#[derive(Template)]
#[template(path = "pages/admin_booking_update.html")]
pub struct AdminBooking1UpdateTemplate {
    pub user_name: Option<String>,
    pub current_lang: String,
    pub booking_id: i32,
    pub accommodation_id: String,
    pub check_in_date: String,
    pub check_out_date: String,
    pub error: String,
}

impl I18nTemplate for AdminBooking1UpdateTemplate {

    fn lang(&self) -> &str {
        &self.current_lang
    }
}

// Admin booking update overview template
#[derive(Template)]
#[template(path = "pages/admin_booking_update_overview.html")]
pub struct AdminBookingUpdateOverviewTemplate {
    pub user_name: Option<String>,
    pub current_lang: String,
    pub success: bool,
    pub accommodation_changed: bool,
    pub old_accommodation: String,
    pub new_accommodation: String,
    pub dates_changed: bool,
    pub old_check_in: String,
    pub old_check_out: String,
    pub new_check_in: String,
    pub new_check_out: String,
    pub accommodation: String,
    pub check_in: String,
    pub check_out: String,
    pub nights: i64,
    pub price_per_night: String,
    pub total_price: String,
}

impl I18nTemplate for AdminBookingUpdateOverviewTemplate {

    fn lang(&self) -> &str {
        &self.current_lang
    }
}

// Admin booking status template
#[derive(Template)]
#[template(path = "pages/admin_booking_status.html")]
pub struct AdminBookingStatusTemplate {
    pub user_name: Option<String>,
    pub current_lang: String,
    pub booking_id: i32,
    pub current_status: String,
}

impl I18nTemplate for AdminBookingStatusTemplate {

    fn lang(&self) -> &str {
        &self.current_lang
    }
}

// Admin customer
// Admin customer read template
#[derive(Template)]
#[template(path = "pages/admin_customer_read.html")]
pub struct AdminCustomerReadTemplate {
    pub user_name: Option<String>,
    pub current_lang: String,
    pub customers: Vec<CustomerRow>,
    pub user_role: String,
    pub success: String,
    pub success_last_name: String,
}

impl I18nTemplate for AdminCustomerReadTemplate {

    fn lang(&self) -> &str {
        &self.current_lang
    }
}

// Admin customer create template
#[derive(Template)]
#[template(path = "pages/admin_customer_create.html")]
pub struct AdminCustomerCreateTemplate {
    pub user_name: Option<String>,
    pub current_lang: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub postal_code: String,
    pub city: String,
    pub error: Option<String>,
}

impl I18nTemplate for AdminCustomerCreateTemplate {

    fn lang(&self) -> &str {
        &self.current_lang
    }
}

// Admin customer update template
#[derive(Template)]
#[template(path = "pages/admin_customer_update.html")]
pub struct AdminCustomerUpdateTemplate {
    pub user_name: Option<String>,
    pub current_lang: String,
    pub customer_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub postal_code: String,
    pub city: String,
    pub error: Option<String>,
}

impl I18nTemplate for AdminCustomerUpdateTemplate {

    fn lang(&self) -> &str {
        &self.current_lang
    }
}