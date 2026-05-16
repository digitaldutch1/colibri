


use askama::Template;
use unic_langid::LanguageIdentifier;





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



// Page template structs and Trait implementations EN/NL
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
#[template(path = "pages/contact.html")]
pub struct ContactTemplate {
    pub user_name: Option<String>,
    pub current_lang: String,
}

impl I18nTemplate for ContactTemplate {
    fn lang(&self) -> &str { &self.current_lang }
}



#[derive(Template)]
#[template(path = "pages/tos.html")]
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