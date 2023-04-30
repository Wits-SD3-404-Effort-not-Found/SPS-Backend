use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct SecurityQuestion {
    pub question_id: i32,
    pub user_answer: String
}
