use crate::db::SPS;

use rocket_db_pools::sqlx;
use rocket_db_pools::Connection;

#[derive(sqlx::FromRow, Debug, Default)]
pub struct Account {
    pub account_id: i32,
    pub email: String,
    pub hashed_password: String,
    pub username: String,
    pub cell_number: String,
    pub profile_photo: String
}

// Depracated -> Was using this but never removed it after I stopped because I was debugging
impl Account {
    pub async fn fetch_account(db: &mut Connection<SPS>, email: String) -> Option<Self> {
        None
    }
}

// Api module to describe account authorization
pub mod api {
    use serde::{Deserialize, Serialize};

    // Json body schema of credential authentication requests
    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct AuthRequest {
        pub email: String,
        pub hashed_password: String,
    }

    // Json body schema of credential authentication responses 
    #[derive(Serialize, Deserialize, Debug)]
    pub struct AuthResponse {
        pub session_token: String,
        pub account_id: i32,
        pub days_to_token_expirary: u32,
        pub new_account: bool,
    }
}
