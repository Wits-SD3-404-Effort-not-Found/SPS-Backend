use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SecurityQuestionsRequest {
    pub email: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SecurityQuestionsResponse {
    pub account_id: i32,
    pub questions: Vec<SecurityQuestion>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SecurityQuestion {
    pub question_id: i32,
    pub question: String,
    pub answer: String,
}
