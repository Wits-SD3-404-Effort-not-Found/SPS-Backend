use rocket::http::Status;

use crate::rocket;
use crate::tests::CLIENT;

#[test]
fn test_fetch_rotations_existing_account_with_rotations_ok() {
    let client_binding = CLIENT.lock().unwrap();
    let response = client_binding
        .get(uri!(super::fetch_rotations(1)))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_some());
}

#[test]
fn test_fetch_rotations_exisiting_account_without_rotations_notfound() {
    let client_binding = CLIENT.lock().unwrap();
    let response = client_binding
        .get(uri!(super::fetch_rotations(2)))
        .dispatch();
    assert_eq!(response.status(), Status::NotFound);
    assert!(response.body().is_some());
}

#[test]
fn test_fetch_rotations_nonexisting_account_not_found() {
    let client_binding = CLIENT.lock().unwrap();
    let response = client_binding
        .get(uri!(super::fetch_rotations(0)))
        .dispatch();
    assert_eq!(response.status(), Status::NotFound);
    assert!(response.body().is_some());
}