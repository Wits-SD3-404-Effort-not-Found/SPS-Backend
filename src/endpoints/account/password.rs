use serde::{Serialize, Deserialize};

use crate::endpoints::account::security_question::SecurityQuestion;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewPasswordRequest {
    pub account_id: i32,
    pub new_password: String,
    pub questions: Vec<SecurityQuestion>
}

