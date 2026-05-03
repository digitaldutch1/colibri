


use askama::Template;

#[derive(Template)]
#[template(path = "pages/public_home.html")]
pub struct HomePublicTemplate {
    pub user_name: Option<String>,
}

#[derive(Template)]
#[template(path = "pages/contact.html")]
pub struct ContactTemplate {
    pub user_name: Option<String>,
}

#[derive(Template)]
#[template(path = "pages/tos.html")]
pub struct TosTemplate {
    pub user_name: Option<String>,
}

#[derive(Template)]
#[template(path = "pages/public_booking1.html")]
pub struct PublicBooking1Template {
    pub user_name: Option<String>,
    pub accommodation_id: String,
}

#[derive(Template)]
#[template(path = "pages/public_booking2.html")]
pub struct PublicBooking2Template {
    pub user_name: Option<String>,
}