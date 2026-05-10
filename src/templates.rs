


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



// Layouts
#[derive(Template)]
#[template(path = "pages/public_home.html")]
pub struct HomePublicTemplate {
    pub user_name: Option<String>,
    pub current_lang: String,

}

impl I18nTemplate for HomePublicTemplate {
    fn lang(&self) -> &str { &self.current_lang }
}



// Pages
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

#[derive(Template)]
#[template(path = "pages/public_booking1.html")]
pub struct PublicBooking1Template {
    pub user_name: Option<String>,
    pub accommodation_id: String,
    pub current_lang: String,
}

impl I18nTemplate for PublicBooking1Template {
    fn lang(&self) -> &str { &self.current_lang }
}

#[derive(Template)]
#[template(path = "pages/public_booking2.html")]
pub struct PublicBooking2Template {
    pub user_name: Option<String>,
    pub current_lang: String,
}

impl I18nTemplate for PublicBooking2Template {
    fn lang(&self) -> &str { &self.current_lang }
}