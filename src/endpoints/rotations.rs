#[cfg(test)]
mod tests;
mod rotation_files;

use rocket::serde::json::Json;
use rocket_db_pools::{
    Connection,
    sqlx
};

use crate::endpoints::errors::{ApiResult, ApiErrors};
use crate::db::{self, SPS};

#[get("/rotations/<account_id>")]
pub async fn fetch_rotations(account_id: i32, mut db_conn: Connection<SPS>) -> ApiResult<Json<Vec<rotation_files::RotationResponse>>>{

    // Checking the user account actually exists
    match sqlx::query!(
        "SELECT account_id FROM tblAccount WHERE account_id = ?",
        account_id
    ).fetch_one(&mut *db_conn).await {
        Ok(_) => (),
        Err(_) => return Err(ApiErrors::NotFound("User account not found".to_string()))
    }

    let db_rotations = match sqlx::query_as!(
        db::Rotation,
        "SELECT * FROM tblEvents JOIN tblRotation USING (event_id) JOIN tblHospital USING (hospital_id) JOIN tblDiscipline USING (discipline_id) WHERE tblEvents.account_id = ?",
        account_id
    ).fetch_all(&mut *db_conn).await {
        Ok(val) => val,
        Err(_) => return Err(ApiErrors::NotFound("No rotations where found".to_string()))
    };

    let rotations: Vec<rotation_files::RotationResponse> = db_rotations.iter().map(|rotation| rotation.into()).collect();

    Ok(Json(rotations))
}
