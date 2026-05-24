


use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Accommodation {
    pub id: i32,
    pub name: String,
    pub total_units: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    pub id: i32,
    pub accommodation_id: i32,
    pub unit_code: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Booking {
    pub id: i32,
    pub customer_id: i32,
    pub accommodation_id: i32,
    pub unit_id: i32,
    pub check_in_date: NaiveDate,
    pub check_out_date: NaiveDate,
    pub total_price: f64,
    pub status: String,
    pub payment_token: Option<String>,
    pub cancel_token: Option<String>,
    pub invoice_number: Option<String>,
    pub source: Option<String>,
    pub external_reference: Option<String>,
    pub created_by_user_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub locked_until: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Availability {
    pub date: NaiveDate,
    pub booked_count: i64,
    pub is_available: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerRow {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
}