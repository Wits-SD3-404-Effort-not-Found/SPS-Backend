pub struct Account {
    pub account_id: u64,
    pub email: String,
    pub hashed_password: String
}

impl Account {
    pub async fn fetch_account(email: String) -> Option<Self> {
        Some(Account {
            account_id: 0,
            email,
            hashed_password: "".to_string()
        })
    }
}

pub mod api {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct AuthRequest {
        pub email: String,
        pub hashed_password: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct AuthResponse {
        pub session_token: String,
        pub account_id: u64,
        pub days_to_token_expirary: u32,
        pub new_account: bool,
    }
}
