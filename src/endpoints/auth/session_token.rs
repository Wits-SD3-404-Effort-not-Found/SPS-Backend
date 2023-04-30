use crate::db::Account;
use crypto::{digest::Digest, sha2::Sha256};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenRequest {
    pub session_token: String,
    pub account_id: u32,
}

pub struct SessionToken {
    pub account_id: i32,
    pub session_token_id: i32,
    pub token: String,
    pub expiry_date: chrono::NaiveDate,
}

// Generate a unique Hash session token based off the account requesting
// the token, and the time it was requested. Helps keep the tokens unique
// even if multiple requests from the same account or same time happen
pub fn generate_session_token(account: &Account) -> SessionToken {
    let timestamp_millis = chrono::Utc::now().timestamp_millis().to_string();
    let email = &account.email;
    let id = account.account_id.to_string();

    let mut hasher = Sha256::new();
    hasher.input_str(format!("{}{}{}", timestamp_millis, email, id).as_str());

    let expiry_date = chrono::Utc::now().naive_utc() + chrono::Duration::days(180);

    SessionToken {
        session_token_id: 0,
        account_id: account.account_id,
        token: hasher.result_str(),
        expiry_date: expiry_date.date(),
    }
}

#[cfg(test)]
mod tests {
    use crate::db::Account;
    use hex;

    #[tokio::test]
    async fn test_generate_session_token_expected_input_correct_token() {
        let mut test_account = Account::default();
        test_account.email = "testEmailAddress@gmail.com".to_string();
        test_account.hashed_password = "test_password+1".to_string();

        let token_result = super::generate_session_token(&test_account);
        let hex_result = hex::decode(&token_result.token); // if this fails then its an invalid hex string
                                                           // and concequently an invalid sha256

        assert!(!hex_result.is_err());
        let hex_str = hex_result.unwrap();
        assert_eq!(hex_str.len(), 32);
    }
}
