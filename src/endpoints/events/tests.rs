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

#[test]
fn test_insert_event_existing_account_ok() {
    let client_binding = CLIENT.lock().unwrap();
    let body = super::event_api::NewEvent {
        account_id: 1,
        start_date: "2023-01-01 00:00:00.000".to_string(),
        end_date: "2023-01-01 00:00:00.000".to_string(),
        event_name: "TEST ROCKET EVENT INSERT".to_string(),
        description: Some("TEST ROCKET EVENT DESCRIPTION".to_string()),
    };
    let response = client_binding.post(uri!(super::add_event))
        .body(serde_json::to_string(&body).unwrap()).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_none());
}

#[test]
fn test_insert_event_nonexisting_account_not_found() {
    let client_binding = CLIENT.lock().unwrap();
    let body = super::event_api::NewEvent {
        account_id: 0,
        start_date: "2023-01-01 00:00:00.000".to_string(),
        end_date: "2023-01-01 00:00:00.000".to_string(),
        event_name: "TEST ROCKET EVENT INSERT".to_string(),
        description: Some("TEST ROCKET EVENT DESCRIPTION".to_string()),
    };
    let response = client_binding.post(uri!(super::add_event))
        .body(serde_json::to_string(&body).unwrap()).dispatch();
    assert_eq!(response.status(), Status::NotFound);
    assert!(response.body().is_some());
}