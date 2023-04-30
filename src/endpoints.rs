pub mod account;
pub mod auth;
pub mod events;
pub mod notes;
pub mod rotations;

mod errors;

use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::Connection;

use crate::db::SPS;

// Code Demo:
//  for defining an index
//  Accessing settings, its async but might need to switched to sync later
#[get("/")]
pub async fn index(mut db: Connection<SPS>) -> &'static str {
    sqlx::query("SELECT * FROM tblAccount")
        .fetch_one(&mut *db)
        .await
        .and_then(|r| dbg!(r.try_get::<String, usize>(1)))
        .ok();

    "Wits Student Placement System API"
}

#[cfg(test)]
mod tests {

    use rocket::http::Status;

    use crate::tests::CLIENT;

    #[test]
    fn test_index_none_ok() {
        let client_binding = CLIENT.lock().unwrap();

        let response = client_binding.get(uri!(super::index)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.body().is_some());
    }
}
