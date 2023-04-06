#[cfg(test)]
mod tests;

pub mod auth;
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

    sqlx::query("SELECT * FROM tblAccount").fetch_one(&mut *db).await
        .and_then(|r| 
            dbg!(r.try_get::<String, usize>(1))
        )
        .ok();

    "Wits Student Placement System API"
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
