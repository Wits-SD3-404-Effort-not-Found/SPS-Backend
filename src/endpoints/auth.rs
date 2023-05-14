//! # Authentication Endpoints
//! All endpoint urls will begin with /authentication/

#[cfg(test)]
mod tests;

mod credentials;
mod security_questions;
mod session_token;

use regex::Regex;
use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection};

use crate::db::{self, SPS};
use crate::endpoints::errors::{ApiErrors, ApiResult};

/// ## Validate email address
///
/// Checks whether or not a given email address is a valid student
/// email address.
fn validate_email(email: &String) -> ApiResult<()> {
    // Regex setup and error handling
    let email_rule_regex = match Regex::new(r"^[0-9]{7}@students.wits.ac.za$") {
        Ok(val) => val,
        Err(e) => {
            return Err(ApiErrors::InternalError(format!(
                "Internal Server Ererrorsror: {}",
                e
            )))
        }
    };

    // Ensure that the received email address is a student account
    if !email_rule_regex.is_match(&email) {
        return Err(ApiErrors::Unauth(
            "Invalid email address provided".to_string(),
        ));
    }

    Ok(())
}

/// ## Authenticate User Credentials
///
/// ### Arguments
///
/// ```json
///     {
///         "email": string,
///         "hashed_password": string
///     }
/// ```
///
/// ### Possible Responses
///
/// * 200 Ok
/// * 401 Unauthorized
#[post("/authentication/credentials", data = "<credentials>")]
pub async fn auth_credentials(
    mut db_conn: Connection<SPS>,
    credentials: Json<credentials::CredentialRequest>,
) -> ApiResult<Json<credentials::CredentialReponse>> {
    validate_email(&credentials.email)?;

    let mut is_new_account = false;
    let db_account = match sqlx::query_as!(
        db::Account,
        "SELECT * FROM tblAccount WHERE email = ?",
        &credentials.email
    )
    .fetch_one(&mut *db_conn)
    .await
    {
        Ok(val) => val,
        Err(_) => {
            // If no account is found, create a new one and then return the account id.
            // Front end can then handle populating the newly created user profile
            // This whole new account stuff is to remove the need for a sign up because
            // techincally students shouldn't need to make a new account(They already have one)
            // but we don't have access to that database just yet. This is a work around that
            // means we can maintain the whole appearance of never signing up while still
            // actually having a sign up process. Its just obfuscated
            is_new_account = true;
            let mut new_account = db::Account::default();
            new_account.email = credentials.email.to_owned();
            new_account.hashed_password = credentials.hashed_password.to_owned();

            match sqlx::query!(
                "INSERT INTO tblAccount(email, hashed_password, username, cell_number) VALUES (?, ?, ?, ?)",
                new_account.email, new_account.hashed_password, "", ""
            ).execute(&mut *db_conn).await {
                Ok(_) => (),
                #[cfg(not(tarpaulin_include))]
                Err(_) => return Err(ApiErrors::InternalError("Unable to create new account".to_string())),
            }

            new_account
        }
    };

    if &db_account.hashed_password != &credentials.hashed_password {
        return Err(ApiErrors::Unauth("Incorrect provided password".to_string()));
    }

    let token = session_token::generate_session_token(&db_account);

    match sqlx::query!(
        "INSERT INTO tblSessionToken (account_id, token, expiry_date) VALUES (?, ?, ?)",
        token.account_id,
        token.token,
        token.expiry_date
    )
    .execute(&mut *db_conn)
    .await
    {
        Ok(_) => (),
        Err(_) => (),
    }

    Ok(Json(credentials::CredentialReponse {
        session_token: token.token,
        account_id: db_account.account_id,
        new_account: is_new_account,
    }))
}

/// ## Send email for forgot password
///
/// ### Arguments
///
/// ```json
///     {
///         "email": string
///     }
/// ```
///
/// ### Possible Responses
///
/// * 200 Ok
/// * 404 Not Found
#[post("/authentication/security_questions", data = "<reset_details>")]
pub async fn auth_security_questions(
    mut db_conn: Connection<SPS>,
    reset_details: Json<security_questions::SecurityQuestionsRequest>,
) -> ApiResult<Json<security_questions::SecurityQuestionsResponse>> {
    validate_email(&reset_details.email)?;

    let account_questions = match sqlx::query!(
        "SELECT tblAccount.email as email, tblAccount.account_id as account_id, tblSecurityAnswers.secques_id as question_id, tblSecurityQuestions.question as question, tblSecurityAnswers.answer as answer FROM tblSecurityAnswers JOIN tblAccount ON tblAccount.account_id = tblSecurityAnswers.account_id JOIN tblSecurityQuestions ON tblSecurityAnswers.secques_id = tblSecurityQuestions.secques_id WHERE tblAccount.email = ?",
        &reset_details.email
    ).fetch_all(&mut *db_conn).await {
        Ok(val) => val,
        Err(_) => return Err(ApiErrors::InternalError("Failed to query database".to_string()))
    };

    match account_questions.len() == 0 {
        true => {
            return Err(ApiErrors::NotFound(
                "Provided email address not found in database".to_string(),
            ))
        }
        _ => (),
    }

    let mut sq_response = security_questions::SecurityQuestionsResponse::default();

    for question in &account_questions {
        sq_response.account_id = question.account_id;
        sq_response
            .questions
            .push(security_questions::SecurityQuestion {
                question_id: question.question_id,
                question: question.question.to_owned(),
                answer: question.answer.to_owned(),
            })
    }

    Ok(Json(sq_response))
}

/// ## Authenticate a session token
///
/// ### Arguments
/// ```json
///     {
///         "account_id": i32,
///         "token": string
///     }
/// ```
///
/// ### Possible Responses
///
/// * 200 Ok
/// * 401 Unauthorized
/// * 404 Not Found
#[post("/authentication/session", data = "<token>")]
pub async fn auth_session(
    mut db_conn: Connection<SPS>,
    token: Json<session_token::TokenRequest>,
) -> ApiResult<()> {
    let token = match sqlx::query_as!(
        db::SessionToken,
        "SELECT * FROM tblSessionToken WHERE account_id = ? AND token = ?",
        token.account_id,
        token.session_token
    )
    .fetch_one(&mut *db_conn)
    .await
    {
        Ok(val) => val,
        Err(_) => return Err(ApiErrors::Unauth("Session Token not found".to_string())),
    };

    // Converting for comparison against the current value
    let expiry_datetime = match token.expiry_date.and_hms_opt(0, 0, 0) {
        Some(val) => val,
        None => chrono::NaiveDateTime::MIN,
    };
    match chrono::Utc::now().naive_utc() > expiry_datetime {
        true => {
            match sqlx::query!(
                "DELETE FROM tblSessionToken WHERE session_token_id = ?",
                token.session_token_id
            )
            .execute(&mut *db_conn)
            .await
            {
                Ok(_) => (),
                Err(_) => {
                    return Err(ApiErrors::InternalError(
                        "Failed to delete expired session token".to_string(),
                    ))
                }
            };

            return Err(ApiErrors::Unauth("Expired Session Token".to_string()));
        }
        false => (),
    };

    Ok(())
}

#[delete("/authentication/session/<token>")]
pub async fn remove_session(token: String, mut db_conn: Connection<SPS>) -> ApiResult<()> {
    let ses_token_id: i32 = match sqlx::query!(
        "SELECT session_token_id FROM tblSessionToken WHERE token = ?",
        token
    )
    .fetch_one(&mut *db_conn)
    .await
    {
        Ok(val) => val.session_token_id,
        Err(_) => return Err(ApiErrors::NotFound("Session token not found".to_string())),
    };

    match sqlx::query!(
        "DELETE FROM tblSessionToken WHERE session_token_id = ?",
        ses_token_id
    )
    .execute(&mut *db_conn)
    .await
    {
        Ok(_) => (),
        Err(_) => {
            return Err(ApiErrors::InternalError(
                "Unable to remove session token from database".to_string(),
            ))
        }
    };

    Ok(())
}
