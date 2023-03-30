#[cfg(test)]
mod tests;

mod auth;
mod errors;

use rocket::serde::json::Json;
use regex::Regex;

use errors::ApiResult;
use crate::{SETTINGS, sps::auth::account::Account};
use crate::sps::auth::account::api;
use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::Connection;
use crate::db::SPS;

// Code Demo:
//  for defining an index
//  Accessing settings, its async but might need to switched to sync later
#[get("/")]
pub async fn index(mut db: Connection<SPS>) -> &'static str {
    let settings = SETTINGS.read().await;
    log::info!("test {}", settings.get::<String>("test_value").unwrap());

    let row = sqlx::query("SELECT * FROM tblAccount").fetch_one(&mut *db).await.unwrap();
    dbg!(row);

    "Wits Student Placement System API"
}

/// The api endpoint for credential authentication. Will return the authentication result
///
/// # Arguments
/// `credentials` -> JSON object holding the username and password of an account
///
/// # Possible Responses
/// 1. 200 Ok - Provided valid authentication credentials
/// 	* Returns a JSON object with the following:
/// 		* session token
/// 		* account id
/// 		* days to session token expirary
/// 2. 401 Unauthorized - Provided invalid authentication credentials
#[post("/authentication/credentials", data = "<credentials>")]
pub async fn auth_credentials(
    credentials: Json<api::AuthRequest>,
) -> ApiResult<Json<api::AuthResponse>> {

    // Regex setup and error handling
    let email_rule_regex = match Regex::new(r"^[0-9]{7}@students.wits.ac.za$") {
        Ok(val) => val,
        Err(e) => return Err(errors::ApiErrors::InternalError(format!("Internal Server Error: {}", e)))
    };

    // Ensure that the received email address is a student account
    if !email_rule_regex.is_match(&credentials.email) {
        return Err(errors::ApiErrors::Unauth("Invalid email address provided".to_string()));
    }

    let mut new_account = false;
    let db_account = match Account::fetch_account(credentials.email.clone()).await {
        None => {
            // If no account is found, create a new one and then return the account id.
            // Front end can then handle populating the newly created user profile
            // This whole new account stuff is to remove the need for a sign up because
            // techincally students shouldn't need to make a new account(They already have one)
            // but we don't have access to that database just yet. This is a work around that
            // means we can maintain the whole appearance of never signing up while still 
            // actually having a sign up process. Its just obfuscated
            log::info!("No database account found for recieved email");
            new_account = true;
            Account {
                account_id: 0,
                email: credentials.email.clone(),
                hashed_password: credentials.email.clone()
            }
        },
        Some(val) => val,
    };

    let token = super::sps::auth::session_token::generate_session_token(&db_account).await;
    Ok(Json(super::sps::auth::account::api::AuthResponse {
        session_token: token.token,
        account_id: db_account.account_id,
        days_to_token_expirary: 180,
        new_account,
    }))
}

/// The api endpoint for session authentication.
/// Will reutrn the authentication result.
///
///	# Arguments
///
/// # Possible Responses
///	1. 200 Ok - Provided valid session token and account id
///	2. 401 Unauthorized - Provided invalid session token or account id
#[post("/authentication/session", data = "<token>")]
pub async fn auth_session(token: Json<auth::session_token::RequestBody>) -> ApiResult<()> {
    dbg!(token);

    Ok(())
}

/// The api endpoint for forgot password and email authentication
/// Will return if the email is found
///
/// # Arguments
///
///	# Possible Responses
///	1. 200 Ok - Provided email address is found in the database
///	2. 404 Not Found - Provided email address is not found in the database
#[post("/authentication/forgot", data = "<email>")]
pub async fn auth_forgot(email: Json<auth::otp::email::RequestBody>) -> ApiResult<()> {
    dbg!(email);

    Ok(())
}

/// The api endpoint for authenticating a one-time-pin
/// Will return the authentication result
///
/// # Arguments
///
///	# Possible Responses
/// 1. 200 Ok - Provided correct OTP value
/// 2. 401 Unauthorized - Provided incorrect OTP value
#[post("/authentication/otp", data = "<otp>")]
pub async fn auth_otp(otp: Json<auth::otp::RequestBody>) -> ApiResult<()> {
    dbg!(otp);

    Ok(())
}
