use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CredentialRequest {
    pub email: String,
    pub hashed_password: String
}

#[derive(Serialize)]
pub struct CredentialReponse {
    pub session_token: String,
    pub account_id: i32,
    pub new_account: bool
}
