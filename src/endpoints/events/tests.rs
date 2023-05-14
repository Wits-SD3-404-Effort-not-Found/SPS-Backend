use rocket::http::Status;

use crate::rocket;
use crate::tests::CLIENT;

#[test]
fn test_fetch_events_existing_account_with_notes_ok() {
    let client_binding = CLIENT.lock().unwrap();
    let response = client_binding.get(uri!(super::fetch_events(1))).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_some());
}

#[test]
fn test_fetch_events_nonexisting_account_not_found() {
    let client_binding = CLIENT.lock().unwrap();
    let response = client_binding.get(uri!(super::fetch_events(0))).dispatch();
    assert_eq!(response.status(), Status::NotFound);
    assert!(response.body().is_some());
}

#[test]
fn test_fetch_events_existing_account_without_events_not_found() {
    let client_binding = CLIENT.lock().unwrap();
    let response = client_binding.get(uri!(super::fetch_events(10))).dispatch();
    assert_eq!(response.status(), Status::NotFound);
    assert!(response.body().is_some());
}