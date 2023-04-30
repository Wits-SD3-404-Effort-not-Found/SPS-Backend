use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SecurityQuestion {
    pub question_id: i32,
    pub user_answer: String,
}
