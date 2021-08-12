use std::str::FromStr;

use chrono::{ DateTime, Utc, NaiveDate};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub birth_date: NaiveDate,
    pub custom_data: CustomData,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(name: String, birth_date_ymd: (i32, u32, u32)) -> Self {
        let (y, m, d) = birth_date_ymd;
        let id = uuid::Uuid::from_str("28b1fe5e-005e-4445-8896-036feb784d96").unwrap();
        Self {
            id,
            name,
            birth_date: NaiveDate::from_ymd(y, m, d),
            custom_data: CustomData { random: 1},
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomData {
    random: u32
}