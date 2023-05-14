use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SecurityQuestion {
    pub question_id: i32,
    pub user_answer: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddSecurityQuestion {
    pub account_id: i32,
    pub questions: Vec<SecurityQuestion>,
}