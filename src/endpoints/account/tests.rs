use rocket::http::Status;

use crate::rocket;
use crate::tests::CLIENT;

#[test]
fn test_fetch_account_existing_account_ok() {
    let client_binding = CLIENT.lock().unwrap();
    let response = client_binding.get(uri!(super::fetch_account(1))).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_some());
}

#[test]
fn test_fetch_account_nonexisting_account_not_found() {
    let client_binding = CLIENT.lock().unwrap();
    let response = client_binding.get(uri!(super::fetch_account(0))).dispatch();
    assert_eq!(response.status(), Status::NotFound);
    assert!(response.body().is_some());
}

#[test]
fn test_update_account_existing_account_ok() {
    let client_binding = CLIENT.lock().unwrap();

    let req_body = super::manage::UpdateAccount {
        account_id: 1,
        username: "Test Username".to_string(),
        cell_number: "0715791902".to_string(),
        profile_photo: vec![0, 0, 0, 0, 0, 0, 0, 0, 0]
    };

    let response = client_binding.put(uri!(super::update_account))
        .body(serde_json::to_string(&req_body).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_none());
}

#[test]
fn test_update_account_nonexisting_account_not_found() {
    let client_binding = CLIENT.lock().unwrap();

    let req_body = super::manage::UpdateAccount {
        account_id: 0,
        username: "Test Username".to_string(),
        cell_number: "0715791902".to_string(),
        profile_photo: vec![0, 0, 0, 0, 0, 0, 0, 0, 0]
    };

    let response = client_binding.put(uri!(super::update_account))
        .body(serde_json::to_string(&req_body).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert!(response.body().is_some());
}
