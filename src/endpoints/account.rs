#[cfg(test)]
mod tests;

mod manage;
mod password;
mod security_question;

use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection};

use crate::db;
use crate::db::SPS;
use crate::endpoints::errors::{ApiErrors, ApiResult};

use self::security_question::AddSecurityQuestion;

/// ## Reset password for an account
///
/// ### Arguments
///
///  * account_id,
///  * questions
///
/// ### Possible Response
///
/// * 200 Ok
/// * 401 Unauthorized
/// * 404 Not Found
#[post("/account/reset_password", data = "<reset_details>")]
pub async fn account_reset_password(
    mut db_conn: Connection<SPS>,
    reset_details: Json<password::NewPasswordRequest>,
) -> ApiResult<()> {
    let _db_account = match sqlx::query_as!(
        db::Account,
        "SELECT * FROM tblAccount WHERE account_id = ?",
        reset_details.account_id
    )
    .fetch_one(&mut *db_conn)
    .await
    {
        Ok(val) => val,
        Err(_) => return Err(ApiErrors::NotFound("Account not found".to_string())),
    };

    let account_questions = match sqlx::query!(
        "SELECT secques_id as question_id, answer as correct_answer FROM tblSecurityAnswers WHERE account_id = ?",
        &reset_details.account_id
    ).fetch_all(&mut *db_conn).await {
        Ok(val) => val,
        Err(_) => return Err(ApiErrors::InternalError("Failed to fetch answers".to_string()))
    };

    for sent_question in &reset_details.questions {
        for account_question in &account_questions {
            match &sent_question.question_id == &account_question.question_id {
                // This could also be done with a hashmap but I think this is easier to both read
                // and write
                // Only check if the answers match if we are looking at the same question
                true => match &sent_question.user_answer == &account_question.correct_answer {
                    false => return Err(ApiErrors::Unauth("Invalid answer provided".to_string())),
                    true => (),
                },
                false => (),
            }
        }
    }

    match sqlx::query!(
        "UPDATE tblAccount SET hashed_password = ? WHERE account_id = ?",
        reset_details.new_password,
        reset_details.account_id
    )
    .execute(&mut *db_conn)
    .await
    {
        #[cfg(not(tarpaulin_include))]
        Err(_) => {
            return Err(ApiErrors::InternalError(
                "Failed to update account password".to_string(),
            ))
        }
        _ => (),
    };

    Ok(())
}

/// ## Update a user's account details
///
/// ### Arguments
///
///  * account_id,
///  * username
///  * cell_number
///  * profile_photo
///
/// ### Possible Response
///
/// * 200 Ok
/// * 404 Not Found
#[put("/account", data = "<updated_account>")]
pub async fn update_account(
    mut db_conn: Connection<SPS>,
    updated_account: Json<manage::UpdateAccount>,
) -> ApiResult<()> {
    let _db_account = match sqlx::query_as!(
        db::Account,
        "SELECT * FROM tblAccount WHERE account_id = ?",
        updated_account.account_id
    )
    .fetch_one(&mut *db_conn)
    .await
    {
        Ok(val) => val,
        Err(_) => return Err(ApiErrors::NotFound("Account not found".to_string())),
    };

    match sqlx::query!(
        "UPDATE tblAccount SET username = ?, cell_number = ?, profile_photo = ? WHERE account_id = ?",
        updated_account.username, updated_account.cell_number, updated_account.profile_photo, updated_account.account_id
    ).execute(&mut *db_conn).await {
        Ok(_) => (),
        #[cfg(not(tarpaulin_include))]
        Err(_) => return Err(ApiErrors::InternalError("Failed to update the account".to_string())),
    };

    Ok(())
}

/// ## Fetch a user's account details
///
/// ### Arguments
///
///  * account_id,
///
/// ### Possible Response
///
/// * 200 Ok
/// * 404 Not Found
#[get("/account/<account_id>")]
pub async fn fetch_account(
    mut db_conn: Connection<SPS>,
    account_id: i32,
) -> ApiResult<Json<manage::UserAccount>> {
    let db_account = match sqlx::query_as!(
        db::Account,
        "SELECT * FROM tblAccount WHERE account_id = ?",
        &account_id,
    )
    .fetch_one(&mut *db_conn)
    .await
    {
        Ok(val) => val,
        Err(_) => {
            return Err(ApiErrors::NotFound(
                "No account with that ID exists".to_string(),
            ))
        }
    };

    let account: manage::UserAccount = db_account.into();

    Ok(Json(account))
}

/// ## Reset a users security questions 
///
/// ### Arguments
///
///  * account_id,
///  * list of security questions
///
/// ### Possible Response
///
/// * 200 Ok
/// * 404 Not Found
#[post("/account/security", data = "<add_questions>")]
pub async fn add_questions(mut db_conn: Connection<SPS>, add_questions: Json<AddSecurityQuestion>) -> ApiResult<()> {
    let db_account = match sqlx::query_as!(
        db::Account,
        "SELECT * FROM tblAccount WHERE account_id = ?",
        &add_questions.account_id,
    )
    .fetch_one(&mut *db_conn)
    .await
    {
        Ok(val) => val,
        Err(_) => {
            return Err(ApiErrors::NotFound(
                "No account with that ID exists".to_string(),
            ))
        }
    };

    match sqlx::query!(
        "DELETE FROM tblSecurityAnswers WHERE account_id = ?",
        add_questions.account_id
    ).execute(&mut *db_conn).await {
        Ok(_) => (),
        #[cfg(not(tarpaulin_include))]
        Err(_) => return Err(ApiErrors::InternalError("Failed to remove old security questions".to_string())),
    };

    for question in &add_questions.questions {
        match sqlx::query!(
            "INSERT INTO tblSecurityAnswers (secques_id, account_id, answer) VALUES (?, ?, ?)",
            question.question_id, add_questions.account_id, question.user_answer
        ).execute(&mut *db_conn).await {
            Ok(_) => (),
            #[cfg(not(tarpaulin_include))]
            Err(_) => return Err(ApiErrors::InternalError("Failed to insert the new questions".to_string()))
        }
    }

    Ok(())
}