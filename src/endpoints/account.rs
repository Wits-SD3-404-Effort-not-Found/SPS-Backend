use rocket::serde::json::Json;
use rocket_db_pools::{
    Connection,
    sqlx
};

use crate::endpoints::errors::{ApiResult, ApiErrors};
use crate::db::{self, SPS};

#[post("/account/reset/password", data = "<reset_details>")]
pub async fn account_reset_password(mut db_conn: Connection<SPS>, reset_details: Json<management::ForgotPasswordRequest>) -> ApiResult<()> {
    // Check email address matches account id
    // Check otp matches account id
    // update password for the account

    let account_details = match sqlx::query!(
            "SELECT tblAccount.id as account_id, tblAccount.email, tblOTP.otp FROM tblAccount INNER JOIN tblOTP on tblOTP.account_id = tblAccount.id"
        ).fetch_one(&mut *db_conn).await {
            Ok(val) => val,
            Err(_) => return Err(ApiErrors::NotFound("Account not found".to_string()))
        };

    if &account_details.email != &reset_details.email {
        // TODO: Clear db OTP
        return Err(ApiErrors::Unauth("Provided email address does not match account email address".to_string()));
    }

    if &account_details.otp != &reset_details.otp {
        // TODO: Clear db OTP
        return Err(ApiErrors::Unauth("Provided OTP does not match generated OTP".to_string()));
    }

    match sqlx::query!("UPDATE tblAccount SET hashed_password = ? WHERE id = ?",
        reset_details.new_password,
        account_details.account_id
    ).execute(&mut *db_conn).await {
        Err(_) => return Err(ApiErrors::InternalError("Failed to update account password".to_string())),
        _ => ()
    };

    Ok(())
}

mod management {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct ForgotPasswordRequest {
        pub account_id: i32,
        pub email: String,
        pub otp: String,
        pub new_password: String,
    }
}
