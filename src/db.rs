use rocket_db_pools::Database;
use rocket_db_pools::sqlx;

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
    pub id: i32,
    pub email: String,
    pub hashed_password: String,
    pub username: String,
    pub cell_number: Option<String>,
    pub profile_photo: Option<Vec<u8>>
}
