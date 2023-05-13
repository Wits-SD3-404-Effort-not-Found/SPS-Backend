mod event_api;
#[cfg(test)]
mod tests;

use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection};

use crate::db::{self, SPS};
use crate::endpoints::errors::{ApiErrors, ApiResult};

/// ## Fetch events for an account
///
/// ### Arguments
///
/// * account id
///
/// ### Possible Response
///
/// * 200 Ok
/// * 404 Not Found
#[get("/events/<account_id>")]
pub async fn fetch_events(
    account_id: i32,
    mut db_conn: Connection<SPS>,
) -> ApiResult<Json<Vec<event_api::EventFile>>> {
    // Checking the user account actually exists
    match sqlx::query!(
        "SELECT account_id FROM tblAccount WHERE account_id = ?",
        account_id
    )
    .fetch_one(&mut *db_conn)
    .await
    {
        Ok(_) => (),
        Err(_) => return Err(ApiErrors::NotFound("User account not found".to_string())),
    }

    let db_events = match sqlx::query_as!(
        db::Event,
        "SELECT * FROM tblEvents WHERE account_id = ?",
        account_id
    )
    .fetch_all(&mut *db_conn)
    .await
    {
        Ok(val) => val,
        Err(_) => return Err(ApiErrors::NotFound("No events where found".to_string())),
    };

    let events: Vec<event_api::EventFile> = db_events.iter().map(|event| event.into()).collect();

    Ok(Json(events))
}


//insert a new record
/// ## Add a event file to an account
///
/// Add a event to an account
///
/// ### Arguments
///
/// * Account ID
/// * New event file
///
/// ### Responses
///
/// * 200 Ok
/// * 404 Not Found
#[post("/events", data = "<new_event>")]
pub async fn add_event(
    new_event: Json<event_api::NewEvent>,
    mut db_conn: Connection<SPS>,
) -> ApiResult<()> {
    // Checking the user account actually exists
    match sqlx::query!(
        "SELECT account_id FROM tblAccount WHERE account_id = ?",
        new_event.account_id
    )
    .fetch_one(&mut *db_conn)
    .await
    {
        Ok(_) => (),
        Err(_) => return Err(ApiErrors::NotFound("User account not found".to_string())),
    }

    match sqlx::query!(
        "INSERT INTO tblEvents (account_id, start_date, end_date, event_name, description) VALUES (?, ?, ?, ?, ?)",
        new_event.account_id,
        new_event.start_date,
        new_event.end_date,
        new_event.event_name, 
        new_event.description
    )
    .execute(&mut *db_conn)
    .await
    {
        Ok(_) => (),
        Err(_) => {
            return Err(ApiErrors::InternalError(
                "Unable to save file in database".to_string(),
            ))
        }
    }

    Ok(())
}

//Update a specific event
/// ## Update a specific event file content
///
/// Update a the content of the event file, not the title
///
/// ### Arguments
///
/// * Event ID
/// * Updated Event file
///
/// ### Responses
///
/// * 200 Ok
/// * 404 Not Found
#[put("/events", data = "<update_event>")]
pub async fn update_event(
    update_event: Json<event_api::UpdateEvent>,
    mut db_conn: Connection<SPS>,
) -> ApiResult<()> {
    // Fetching the event record
    let _db_event = match sqlx::query_as!(
        db::Event,
        "SELECT * FROM tblEvents WHERE event_id = ?",
        update_event.event_id
    )
    .fetch_one(&mut *db_conn)
    .await
    {
        Ok(val) => val,
        Err(_) => return Err(ApiErrors::NotFound("Event not found".to_string())),
    };

    // Updating the record
    match sqlx::query!(
        "UPDATE tblEvents SET start_date = ?, end_date = ?, event_name = ?, description = ? WHERE event_id = ?",
        update_event.start_date,
        update_event.end_date,
        update_event.event_name,
        update_event.description,
        update_event.event_id
    )
    .execute(&mut *db_conn)
    .await
    {
        Ok(_) => (),
        Err(_) => {
            return Err(ApiErrors::InternalError(
                "Failed to update the event".to_string(),
            ))
        }
    };

    Ok(())
}

/// ## Delete a event file
///
/// Removes both the database record, and the static file for the event
///
/// ### Arguments
///
/// * Event ID
///
/// ### Responses
///
/// * 200 Ok
/// * 404 Not Found
#[delete("/events/<event_id>")]
pub async fn remove_event(event_id: i32, mut db_conn: Connection<SPS>) -> ApiResult<()> {
    // Fetching the event record
    let _db_event = match sqlx::query_as!(
        db::Event,
        "SELECT * FROM tblEvents WHERE event_id = ?",
        event_id
    )
    .fetch_one(&mut *db_conn)
    .await
    {
        Ok(val) => val,
        Err(_) => return Err(ApiErrors::NotFound("Event not found".to_string())),
    };

    match sqlx::query!("DELETE FROM tblEvents WHERE event_id = ?", event_id)
        .execute(&mut *db_conn)
        .await
    {
        Ok(_) => (),
        Err(_) => {
            return Err(ApiErrors::InternalError(
                "Unable to remove file from database".to_string(),
            ))
        }
    }

    Ok(())
}