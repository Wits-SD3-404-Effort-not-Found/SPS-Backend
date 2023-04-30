mod password;
mod security_question;
mod manage;

use rocket::serde::json::Json;
use rocket_db_pools::{
    Connection,
    sqlx
};

use crate::endpoints::errors::{ApiResult, ApiErrors};
use crate::db::SPS;
use crate::db;

#[post("/account/reset_password", data = "<reset_details>")]
pub async fn account_reset_password(mut db_conn: Connection<SPS>, reset_details: Json<password::NewPasswordRequest>) -> ApiResult<()> {
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

#[put("/account", data = "<updated_account>")]
pub async fn update_account(mut db_conn: Connection<SPS>, updated_account: Json<manage::UpdateAccount>) -> ApiResult<()> {

    let _db_account = match sqlx::query_as!(
        db::Account,
        "SELECT * FROM tblAccount WHERE account_id = ?",
        updated_account.account_id
    ).fetch_one(&mut *db_conn).await {
        Ok(val) => val,
        Err(_) => return Err(ApiErrors::NotFound("Account not found".to_string()))
    };

    match sqlx::query!(
        "UPDATE tblAccount SET username = ?, cell_number = ?, profile_photo = ? WHERE account_id = ?",
        updated_account.username, updated_account.cell_number, updated_account.profile_photo, updated_account.account_id
    ).execute(&mut *db_conn).await {
        Ok(_) => (),
        Err(_) => return Err(ApiErrors::InternalError("Failed to update the account".to_string())),
    };

    Ok(())
}

#[get("/account/<account_id>")]
pub async fn fetch_account(mut db_conn: Connection<SPS>, account_id: i32) -> ApiResult<Json<manage::UserAccount>> {
    let db_account = match sqlx::query_as!(
        db::Account,
        "SELECT * FROM tblAccount WHERE account_id = ?",
        &account_id,
    ).fetch_one(&mut *db_conn).await {
        Ok(val) => val,
        Err(_) => return Err(ApiErrors::NotFound("No account with that ID exists".to_string()))
    };

    let account: manage::UserAccount = db_account.into();

    Ok(Json(account))
}

