#[cfg(test)]
mod tests;
mod event_file;

use rocket::serde::json::Json;
use rocket_db_pools::{
    Connection,
    sqlx
};

use crate::endpoints::errors::{ApiResult, ApiErrors};
use crate::db::{self, SPS};

#[get("/events/<account_id>")]
pub async fn fetch_events(account_id: i32, mut db_conn: Connection<SPS>) -> ApiResult<Json<Vec<event_file::EventFile>>>{

    // Checking the user account actually exists
    match sqlx::query!(
        "SELECT account_id FROM tblAccount WHERE account_id = ?",
        account_id
    ).fetch_one(&mut *db_conn).await {
        Ok(_) => (),
        Err(_) => return Err(ApiErrors::NotFound("User account not found".to_string()))
    }

    let db_events = match sqlx::query_as!(
        db::Event,
        "SELECT * FROM tblEvents WHERE account_id = ?",
        account_id
    ).fetch_all(&mut *db_conn).await {
        Ok(val) => val,
        Err(_) => return Err(ApiErrors::NotFound("No events where found".to_string()))
    };

    let events: Vec<event_file::EventFile> = db_events.iter().map(|event| event.into()).collect();

    Ok(Json(events))
}