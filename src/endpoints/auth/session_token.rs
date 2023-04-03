use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    pub session_token: String,
    pub account_id: u32,
}
