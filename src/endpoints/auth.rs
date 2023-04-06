//! # Authentication Endpoints
//! All endpoint urls will begin with /authentication/

mod credentials;
pub mod otp;
pub mod session_token;

use rocket::serde::json::Json;
use rocket_db_pools::{
    Connection,
    sqlx
};
use regex::Regex;

use crate::endpoints::errors::{ApiResult, ApiErrors};
use crate::db::{self, SPS};

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
pub async fn auth_credentials(mut db_conn: Connection<SPS>, credentials: Json<credentials::CredentialRequest>) -> ApiResult<Json<credentials::CredentialReponse>> {
    // Regex setup and error handling
    let email_rule_regex = match Regex::new(r"^[0-9]{7}@students.wits.ac.za$") {
        Ok(val) => val,
        Err(e) => return Err(ApiErrors::InternalError(format!("Internal Server Ererrorsror: {}", e)))
    };

    // Ensure that the received email address is a student account
    if !email_rule_regex.is_match(&credentials.email) {
        return Err(ApiErrors::Unauth("Invalid email address provided".to_string()));
    }

    let mut is_new_account = false;
    let db_account = match sqlx::query_as!(
            db::Account,
            "SELECT * FROM tblAccount WHERE email = ?",
            &credentials.email
        )
        .fetch_one(&mut *db_conn)
        .await {
        Ok(val) => val,
        Err(_) => {

                // If no account is found, create a new one and then return the account id.
                // Front end can then handle populating the newly created user profile
                // This whole new account stuff is to remove the need for a sign up because
                // techincally students shouldn't need to make a new account(They already have one)
                // but we don't have access to that database just yet. This is a work around that
                // means we can maintain the whole appearance of never signing up while still
                // actually having a sign up process. Its just obfuscated
                log::info!("No database account found for received email");

                is_new_account = true;
                let mut new_account = db::Account::default();
                new_account.email = credentials.email.to_owned();
                new_account.hashed_password = credentials.hashed_password.to_owned();
                new_account
            }
    };

    if &db_account .hashed_password != &credentials.hashed_password {
        return Err(ApiErrors::Unauth("Incorrect provided password".to_string()))
    }

    let token = "".to_string(); //TODO
    Ok(Json(credentials::CredentialReponse {
        session_token: token,
        account_id: db_account.id,
        new_account: is_new_account
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
#[post("/authentication/forgot", data = "<forgot>")]
pub async fn auth_forgot(forgot: Json<otp::ForgotRequest>) -> ApiResult<Json<otp::ForgotResponse>> {
    todo!()
}
