use rocket_db_pools::Database;
use rocket_db_pools::sqlx;
use serde::Serialize;

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

#[derive(Debug, Default, Serialize)]
pub struct Protocol {
    pub protocol_id: i32,
    pub title: String,
    pub content: Option<String>,
}
