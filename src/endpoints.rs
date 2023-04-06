#[cfg(test)]
mod tests;

pub mod auth;
mod errors;

use rocket::serde::json::Json;
use regex::Regex;

use errors::ApiResult;
use crate::{SETTINGS, sps::auth::account::Account};
use crate::sps::auth::account::api;
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

    sqlx::query("SELECT * FROM tblAccount").fetch_one(&mut *db).await
        .and_then(|r| 
            dbg!(r.try_get::<String, usize>(1))
        )
        .ok();

    "Wits Student Placement System API"
}
