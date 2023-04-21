pub mod auth;
pub mod account;
pub mod notes;
mod errors;

use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::Connection;

use crate::db::SPS;
use crate::SETTINGS;

// Code Demo:
//  for defining an index
//  Accessing settings, its async but might need to switched to sync later
#[get("/")]
pub async fn index(mut db: Connection<SPS>) -> &'static str {
    sqlx::query("SELECT * FROM tblAccount").fetch_one(&mut *db).await
        .and_then(|r| 
            dbg!(r.try_get::<String, usize>(1))
        )
        .ok();

    "Wits Student Placement System API"
}

#[cfg(test)]
mod tests {

    use rocket::local::blocking::Client;
    use rocket::http::Status;
    
    #[test]
    fn test_index_none_ok() {
        let client = Client::tracked(crate::rocket()).expect("valid rocket instance");

        let response = client.get(uri!(super::index)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.body().is_some());
    }
}
