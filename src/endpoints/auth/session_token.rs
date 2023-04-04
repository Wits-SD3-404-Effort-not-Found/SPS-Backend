use serde::{Deserialize, Serialize};

// THis should be moved and renamed. Thought I did that already but
// apparently not. I'll fix it sprint 2
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    pub session_token: String,
    pub account_id: u32,
}
