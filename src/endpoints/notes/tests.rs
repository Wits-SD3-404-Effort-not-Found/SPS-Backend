use rocket::http::Status;

use crate::rocket;

#[test]
fn test_fetch_protocols_none_ok() {
    let client_binding = crate::tests::CLIENT.blocking_lock();
    let response = client_binding.get(uri!(super::fetch_protocols)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_some());
}

#[test]
fn test_fetch_notes_existing_account_with_notes_ok() {
    let client_binding = CLIENT.blocking_lock();
    let response = client_binding.get(uri!(super::fetch_notes(1))).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_some());
}

#[test]
fn test_fetch_notes_non_existing_account_not_found() {
    let client_binding = CLIENT.blocking_lock();
    let response = client_binding.get(uri!(super::fetch_notes(0))).dispatch();
    assert_eq!(response.status(), Status::NotFound);
    assert!(response.body().is_some());
}
