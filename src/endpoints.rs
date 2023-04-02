#[cfg(test)] mod tests;

use crate::SETTINGS;
use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::Connection;
use crate::db::SPS;

// Code Demo:
//  for defining an index
//  Accessing settings, its async but might need to switched to sync later
#[get("/")]
pub async fn index(mut db: Connection<SPS>) -> &'static str {
    let settings = SETTINGS.read().await;
    log::info!("test {}", settings.get::<String>("test_value").unwrap());

    let row = sqlx::query("SELECT * FROM tblAccount").fetch_one(&mut *db).await.unwrap();
    dbg!(row);

    "Wits Student Placement System API"
}

