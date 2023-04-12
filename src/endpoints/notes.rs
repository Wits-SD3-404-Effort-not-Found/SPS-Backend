//! # Notes Endpoints
//! All endpoint urls will begin with /notes/

#[cfg(test)]
mod tests;
 
use rocket::serde::json::Json;
use rocket_db_pools::{
    Connection,
    sqlx
};

use crate::endpoints::errors::{ApiResult, ApiErrors};
use crate::db::{self, SPS};

/// ## Fetch Emergency Protocols
///
/// Return all the emergency protocols stored in the database
///
/// ### Arguments
///
/// * None
///
/// ### Possible Responses
///
/// * 200 Ok
/// * 404 Not Found
#[get("/notes/protocols")]
pub async fn fetch_protocols(mut db_conn: Connection<SPS>) -> ApiResult<Json<Vec<db::Protocol>>> {
    let db_protocols = match sqlx::query_as!(
        db::Protocol,
        "SELECT protocol_id, title, content FROM tblProtocol",
    ).fetch_all(&mut *db_conn).await {
        Ok(val) => val,
        Err(_) => return Err(ApiErrors::InternalError("Failed to fetch protocols".to_string()))
    };

    if db_protocols.is_empty() {
        return Err(ApiErrors::NotFound("No protocols were found".to_string()))
    }

    Ok(Json(db_protocols))
}
