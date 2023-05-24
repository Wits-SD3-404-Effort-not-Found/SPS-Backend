use rocket_db_pools::sqlx;
use rocket_db_pools::Database;
use serde::{Serialize, Deserialize};

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
    pub profile_photo: Option<Vec<u8>>,
}

/// SQL Table schema for tblEvents
pub struct Event {
    pub account_id: i32,
    pub event_id: i32,
    pub start_date: chrono::NaiveDateTime,
    pub end_date: chrono::NaiveDateTime,
    pub event_name: String,
    pub description: Option<String>,
}

/// SQL Table schema for tblRotations
pub struct Rotation {
    pub account_id: i32,
    pub event_id: i32,
    pub rotation_id: i32,
    pub hospital_id: i32,
    pub discipline_id: i32,
    pub start_date: chrono::NaiveDateTime,
    pub end_date: chrono::NaiveDateTime,
    pub event_name: String,
    pub description: Option<String>,
    pub hospital_name: String,
    pub discipline_name: String,
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
    pub title: String,
    pub public: bool,
}

pub struct SessionToken {
    pub session_token_id: i32,
    pub account_id: i32,
    pub token: String,
    pub expiry_date: chrono::NaiveDate,
}

#[derive(Serialize, Deserialize)]
pub struct SecurityQuestion {
    pub secques_id: i32,
    pub question: String,
}

#[derive(Serialize, Deserialize)]
pub struct Staff {
    pub staff_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub cell_number: String
}