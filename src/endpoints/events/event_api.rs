use serde::{Serialize, Deserialize};

use crate::db;
use chrono::{Utc, TimeZone};

#[derive(Serialize, Deserialize)]
pub struct EventFile {
    pub account_id: i32,
    pub event_id: i32,
    pub start_date: String,
    pub end_date: String,
    pub event_name: String,
    pub description: Option<String>
}

impl From<&db::Event> for EventFile {
    fn from(value: &db::Event) -> Self {
        let description = match &value.description{
            Some(val) => Some(val.clone()),
            None => None
        };

        EventFile {
            account_id: value.account_id,
            event_id: value.event_id,
            start_date: Utc.from_utc_datetime(&value.start_date).to_string(),
            end_date: Utc.from_utc_datetime(&value.end_date).to_string(),
            event_name: value.event_name.clone(),
            description
        }
    }
}
