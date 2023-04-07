use rocket::serde::json::Json;
use rocket_db_pools::{
    Connection,
    sqlx
};

use crate::endpoints::errors::{ApiResult, ApiErrors};
use crate::db::SPS;

#[post("/account/reset_password", data = "<reset_details>")]
pub async fn account_reset_password(mut db_conn: Connection<SPS>, reset_details: Json<management::NewPasswordRequest>) -> ApiResult<()> {
    let account_questions = match sqlx::query!(
        "SELECT secques_id as question_id, answer as correct_answer FROM tblSecurityAnswers WHERE account_id = ?",
        &reset_details.account_id
    ).fetch_all(&mut *db_conn).await {
        Ok(val) => val,
        Err(_) => return Err(ApiErrors::NotFound("Account not found".to_string()))
    };

    for sent_question in &reset_details.questions {
        for account_question in &account_questions {
            match &sent_question.question_id == &account_question.question_id {
                // This could also be done with a hashmap but I think this is easier to both read
                // and write
                // Only check if the answers match if we are looking at the same question
                true => match &sent_question.user_answer == &account_question.correct_answer {
                    false => return Err(ApiErrors::Unauth("Invalid answer provided".to_string())),
                    true => ()
                },
                false => ()
            }
        }
    }

    match sqlx::query!("UPDATE tblAccount SET hashed_password = ? WHERE account_id = ?",
        reset_details.new_password,
        reset_details.account_id
    ).execute(&mut *db_conn).await {
        Err(_) => return Err(ApiErrors::InternalError("Failed to update account password".to_string())),
        _ => ()
    };

    Ok(())
}

mod management {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct NewPasswordRequest {
        pub account_id: i32,
        pub new_password: String,
        pub questions: Vec<SecurityQuestion>
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SecurityQuestion {
        pub question_id: i32,
        pub user_answer: String
    }
}
