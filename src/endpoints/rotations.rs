mod rotation_api;
#[cfg(test)]
mod tests;

use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection};

use crate::db::{self, SPS};
use crate::endpoints::errors::{ApiErrors, ApiResult};

/// ## Fetch rotations for an account
///
/// ### Arguments
///
/// * account id
///
/// ### Possible Responses
///
/// * 200 Ok
/// * 404 Not Found
#[get("/rotations/<account_id>")]
pub async fn fetch_rotations(
    account_id: i32,
    mut db_conn: Connection<SPS>,
) -> ApiResult<Json<Vec<rotation_api::RotationResponse>>> {
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

    let db_rotations = match sqlx::query_as!(
        db::Rotation,
        "SELECT * FROM tblEvents JOIN tblRotation USING (event_id) JOIN tblHospital USING (hospital_id) JOIN tblDiscipline USING (discipline_id) WHERE tblEvents.account_id = ?",
        account_id
    ).fetch_all(&mut *db_conn).await {
        Ok(val) => val,
        #[cfg(not(tarpaulin_include))]
        Err(_) => {
            return Err(ApiErrors::InternalError("Failed to fetch rotations".to_string()))
        }
    };

    if db_rotations.len() == 0 {
        return Err(ApiErrors::NotFound("No rotations where found".to_string()));
    }

    let rotations: Vec<rotation_api::RotationResponse> = db_rotations
        .iter()
        .map(|rotation| rotation.into())
        .collect();

    Ok(Json(rotations))
}
