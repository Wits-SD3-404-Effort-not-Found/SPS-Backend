#[cfg(test)]
mod tests;

use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection};

use crate::db::{self, SPS};
use crate::endpoints::errors::{ApiErrors, ApiResult};

/// ## Fetch Staff details
/// 
/// Return a list of all the staff contact details
/// 
/// ### Arguments
/// 
/// * None
/// 
/// ### Possible Responses
/// 
/// * 200 Ok
/// * 404 Not Found
#[get("/staff")]
pub async fn fetch_staff(mut db_conn: Connection<SPS>) -> ApiResult<Json<Vec<db::Staff>>> {
    let db_staff = match sqlx::query_as!(
        db::Staff,
        "SELECT * FROM tblStaff",
    )
    .fetch_all(&mut *db_conn)
    .await {
        Ok(val) => val,
        #[cfg(not(tarpaulin_include))]
        Err(_) => {
            return Err(ApiErrors::InternalError("Failed to fetch staff".to_string()));
        }
    };

    if db_staff.is_empty() {
        return Err(ApiErrors::NotFound("No staff members were found".to_string()));
    }

    Ok(Json(db_staff))
}