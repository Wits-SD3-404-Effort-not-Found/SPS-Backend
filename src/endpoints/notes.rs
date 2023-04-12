//! # Notes Endpoints
//! All endpoint urls will begin with /notes/

#[cfg(test)]
mod tests;

mod note_files;
 
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

/// ## Fetch List of Notes
///
/// Returns a list of note ID's and the URL to the static file
///
/// ### Arguments
/// 
/// * Account ID
///
/// ### Possible Responses
///
/// * 200 Ok
/// * 404 Not Found
#[get("/notes/<account_id>")]
pub async fn fetch_notes(account_id: i32, mut db_conn: Connection<SPS>) -> ApiResult<Json<Vec<note_files::NoteFile>>> {

    // Checking the user account actually exists
    match sqlx::query!(
        "SELECT account_id FROM tblAccount WHERE account_id = ?",
        account_id
    ).fetch_one(&mut *db_conn).await {
        Ok(_) => (),
        Err(_) => return Err(ApiErrors::NotFound("User account not found".to_string()))
    }

    let db_notes = match sqlx::query_as!(
        db::Note,
        "SELECT * FROM tblNotes WHERE account_id = ?",
        account_id
    ).fetch_all(&mut *db_conn).await {
        Ok(val) => val,
        Err(_) => return Err(ApiErrors::NotFound("No notes where found".to_string()))
    };

    let notes: Vec<note_files::NoteFile> = db_notes.iter().map(|note| note.into()).collect();

    Ok(Json(notes))
}
