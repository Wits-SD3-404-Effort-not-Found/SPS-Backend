use rocket_db_pools::Database;
use rocket_db_pools::sqlx;
use serde::Serialize;

use chrono::{DateTime, Utc};

// Rocket DB integration setup 
#[derive(Database)]
#[database("sps_mysql")]
pub struct SPS(sqlx::MySqlPool);

/// SQL Table schema for tblAccount
///
/// Note:
/// > profile_photo is a blob in the database
#[derive(sqlx::FromRow, Debug, Default)]
pub struct Account {
    pub account_id: i32,
    pub email: String,
    pub hashed_password: String,
    pub username: String,
    pub cell_number: Option<String>,
    pub profile_photo: Option<Vec<u8>>
}

/// SQL Table schema for tblEvents
pub struct Event {
    pub account_id: i32,
    pub event_id: i32,
    pub start_date: chrono::NaiveDateTime,
    pub end_date: chrono::NaiveDateTime,
    pub name: String,
    pub description: Option<String>
}

/// SQL Table schema for tblRotations
pub struct Rotation {
    pub account_id: i32,
    pub event_id: i32,
    pub rotation_id: i32,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub name: String,
    pub description: String,
    pub hospital_name: String,
    pub dicipline_name: String,
}

#[derive(Debug, Default, Serialize)]
pub struct Protocol {
    pub protocol_id: i32,
    pub title: String,
    pub content: Option<String>,
}

/// SQL Table schema for tblNotes
pub struct Note {
    pub note_id: i32,
    pub account_id: i32,
    pub content: String,
    pub title: String
}
