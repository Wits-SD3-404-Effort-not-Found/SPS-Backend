use serde::{Deserialize, Serialize};

use crate::db;

#[derive(Serialize, Deserialize)]
pub struct UserAccount {
    pub username: String,
    pub cell_number: String,
    pub profile_photo: Vec<u8>,
}

impl From<db::Account> for UserAccount {
    fn from(value: db::Account) -> Self {
        let cell = match value.cell_number {
            Some(val) => val,
            None => "0000000000".to_string(),
        };

        let photo: Vec<u8> = match value.profile_photo {
            Some(val) => val,
            None => vec![],
        };

        UserAccount {
            username: value.username.clone(),
            cell_number: cell,
            profile_photo: photo,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateAccount {
    pub account_id: i32,
    pub username: String,
    pub cell_number: String,
    pub profile_photo: Vec<u8>,
}
