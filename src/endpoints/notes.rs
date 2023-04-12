//! # Notes Endpoints
//! All endpoint urls will begin with /notes/

#[cfg(test)]
mod tests;

mod note_files;
 
use rocket::serde::json::Json;
use rocket::fs::TempFile;
use rocket_db_pools::{
    Connection,
    sqlx
};
use uuid::Uuid;

use crate::SETTINGS;
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
#[post("/notes/<account_id>", format = "plain", data = "<note_file>")]
pub async fn add_note(account_id: i32, mut note_file: TempFile<'_>, mut db_conn: Connection<SPS>) -> ApiResult<()> {

    // Checking the user account actually exists
    match sqlx::query!(
        "SELECT account_id FROM tblAccount WHERE account_id = ?",
        account_id
    ).fetch_one(&mut *db_conn).await {
        Ok(_) => (),
        Err(_) => return Err(ApiErrors::NotFound("User account not found".to_string()))
    }

    let file_uuid = Uuid::new_v4();
    let mut temp_buffer = Uuid::encode_buffer();
    let file_name = file_uuid.as_simple().encode_lower(&mut temp_buffer);

    // Getting the specified static file directory
    let settings = SETTINGS.read().await;
    let static_dir = match settings.get::<String>("static_file_directory") {
        Ok(val) => val,
        Err(_) => { 
            return Err(ApiErrors::InternalError("Unable to find static file directory".to_string()))
        }
    };
    
    let note_file_path = format!("{}/{}.md", &static_dir, &file_name); 
    let note_file_url = format!("static/{}.md", &file_name);

    match note_file.persist_to(&note_file_path).await {
        Ok(_) => (),
        Err(_) => return Err(ApiErrors::InternalError("Unable to save file".to_string()))
    }

    match sqlx::query!(
        "INSERT INTO tblNotes (account_id, url) VALUES (?, ?)",
        account_id, 
        note_file_url
    ).execute(&mut *db_conn).await {
        Ok(_) => (),
        Err(_) => return Err(ApiErrors::InternalError("Unable to save file in database".to_string()))
    }

    Ok(())
}


/// ## Update a specific notes file
///
/// Updates a specific notes file
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
#[put("/notes/<note_id>", format = "plain", data="<note_file>")]
pub async fn update_note(note_id: i32, mut note_file: TempFile<'_>,  mut db_conn: Connection<SPS>) -> ApiResult<()> {
    todo!()
}
