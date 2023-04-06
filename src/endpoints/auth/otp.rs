use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ForgotRequest {
    pub email: String
}

#[derive(Serialize, Deserialize)]
pub struct ForgotResponse {
    pub account_id: i32
}
