#[cfg(test)]
mod tests;

use rocket::serde::json::Json;
use rocket_db_pools::Connection;

use crate::db;
use crate::db::SPS;
use crate::endpoints::errors::{ApiErrors, ApiResult};

/// ## Fetch all security questions in the database
/// 
/// ### Arguments
/// 
/// * None
/// 
/// ### Possible Responses
/// 
/// * 200 Ok
/// * 404 Not Found
#[get("/security/questions")]
pub async fn fetch_all_security_questions(mut db_conn: Connection<SPS>) -> ApiResult<Json<Vec<db::SecurityQuestion>>> {
    let db_questions = match sqlx::query_as!(
        db::SecurityQuestion,
        "SELECT * FROM tblSecurityQuestions"
    ).fetch_all(&mut *db_conn).await {
        Ok(val) => val,
        Err(_) => {
            return Err(ApiErrors::InternalError("Failed to fetch all security questions".to_string()))
        }
    };

    if db_questions.is_empty() {
        return Err(ApiErrors::NotFound("No security questions were found".to_string()));
    }

    Ok(Json(db_questions))
}