


use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Accommodation {
    pub id: i32,
    pub name: String,
    pub total_units: i32,
    pub price_per_night: String,
    pub created_at: NaiveDateTime,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct StaffRow {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role: String,
}