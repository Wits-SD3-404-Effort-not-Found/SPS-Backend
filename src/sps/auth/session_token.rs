use crypto::{ sha2::Sha256, digest::Digest };
use std::ops::Add;
use chrono::{ 
    prelude::*,
    naive::Days 
};

use super::account::Account;

pub struct SessionToken {
    pub account_id: i32,
    pub token: String,
    pub expiray_date: DateTime<Utc>,
} 

pub async fn generate_session_token(account: &Account) -> SessionToken {
    let timestamp_millis = Utc::now().timestamp_millis().to_string();
    let email = &account.email;
    let id = account.account_id.to_string();

    let mut hasher = Sha256::new();
    hasher.input_str(format!("{}{}{}", timestamp_millis, email, id).as_str());

    SessionToken {
        account_id: account.account_id,
        token: hasher.result_str(),
        expiray_date: Utc::now().add(Days::new(180))
    }
}
