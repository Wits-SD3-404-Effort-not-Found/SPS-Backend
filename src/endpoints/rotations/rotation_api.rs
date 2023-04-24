use serde::{Serialize, Deserialize};

use crate::db;
use chrono::{Utc, TimeZone};

#[derive(Serialize, Deserialize)]
pub struct RotationResponse {
    pub account_id: i32,
    pub event_id: i32,
    pub rotation_id: i32,
    pub start_date: String,
    pub end_date: String,
    pub event_name: String,
    pub description: Option<String>,
    pub hospital_name: String,
    pub discipline_name: String
}

impl From<&db::Rotation> for RotationResponse {
    fn from(value: &db::Rotation) -> Self {
        
        let description = match &value.description{
            Some(val) => Some(val.clone()),
            None => None
        };

        RotationResponse{
            account_id: value.account_id,
            event_id: value.event_id,
            rotation_id: value.rotation_id,
            start_date: Utc.from_utc_datetime(&value.start_date).to_string(),
            end_date: Utc.from_utc_datetime(&value.end_date).to_string(),
            event_name: value.event_name.clone(),
            description,
            hospital_name: value.hospital_name.clone(),
            discipline_name: value.discipline_name.clone()
        }
    }
}
