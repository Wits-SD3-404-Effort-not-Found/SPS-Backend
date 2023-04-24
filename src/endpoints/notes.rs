//! # Notes Endpoints
//! All endpoint urls will begin with /notes/

#[cfg(test)]
mod tests;

mod note_api;
 
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
pub async fn fetch_notes(account_id: i32, mut db_conn: Connection<SPS>) -> ApiResult<Json<Vec<note_api::NoteResponse>>> {
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

    let notes: Vec<note_api::NoteResponse> = db_notes.iter().map(|note| note.into()).collect();

    Ok(Json(notes))
}

/// ## Add a note file to an account 
///
/// Add a note to an account
///
/// ### Arguments
///
/// * Account ID
/// * New note file
///
/// ### Responses
///
/// * 200 Ok
/// * 404 Not Found
#[post("/notes", data = "<new_note>")]
pub async fn add_note(new_note: Json<note_api::NewNote> , mut db_conn: Connection<SPS>) -> ApiResult<()> {
    // Checking the user account actually exists
    match sqlx::query!(
        "SELECT account_id FROM tblAccount WHERE account_id = ?",
        new_note.account_id
    ).fetch_one(&mut *db_conn).await {
        Ok(_) => (),
        Err(_) => return Err(ApiErrors::NotFound("User account not found".to_string()))
    }

    match sqlx::query!(
        "INSERT INTO tblNotes (account_id, content, title) VALUES (?, ?, ?)",
        new_note.account_id, 
        new_note.note_content,
        new_note.note_title,
    ).execute(&mut *db_conn).await {
        Ok(_) => (),
        Err(_) => return Err(ApiErrors::InternalError("Unable to save file in database".to_string()))
    }

    Ok(())
}


/// ## Update a specific notes file content
///
/// Update a the content of the note file, not the title
///
/// ### Arguments
///
/// * Note ID
/// * Updated note file
///
/// ### Responses
///
/// * 200 Ok
/// * 404 Not Found
#[put("/notes", data="<update_note>")]
pub async fn update_note(update_note: Json<note_api::UpdateNote>, mut db_conn: Connection<SPS>) -> ApiResult<()> {

    // Fetching the notes record
    let _db_note = match sqlx::query_as!(
        db::Note,
        "SELECT * FROM tblNotes WHERE note_id = ?",
        update_note.note_id
    ).fetch_one(&mut *db_conn).await {
        Ok(val) => val,
        Err(_) => return Err(ApiErrors::NotFound("Note not found".to_string()))
    };

    // Updating the recrod
    match sqlx::query!(
        "UPDATE tblNotes SET title = ?, content = ? WHERE note_id = ?",
        update_note.note_title, update_note.note_content, update_note.note_id
    ).execute(&mut *db_conn).await {
        Ok(_) => (),
        Err(_) => return Err(ApiErrors::InternalError("Failed to update the note".to_string())),
    };

    Ok(())
}

/// ## Delete a notes file
///
/// Removes both the database record, and the static file for the note
///
/// ### Arguments
///
/// * Note ID
///
/// ### Responses
///
/// * 200 Ok
/// * 404 Not Found
#[delete("/notes/<note_id>")]
pub async fn remove_note(note_id: i32, mut db_conn: Connection<SPS>) -> ApiResult<()> {

    // Fetching the notes record
    let _db_note = match sqlx::query_as!(
        db::Note,
        "SELECT * FROM tblNotes WHERE note_id = ?",
        note_id 
    ).fetch_one(&mut *db_conn).await {
        Ok(val) => val,
        Err(_) => return Err(ApiErrors::NotFound("Note not found".to_string()))
    };

    match sqlx::query!(
        "DELETE FROM tblNotes WHERE note_id = ?",
        note_id
    ).execute(&mut *db_conn).await {
        Ok(_) => (),
        Err(_) => return Err(ApiErrors::InternalError("Unable to remove file from database".to_string()))
    }

    Ok(())
}
