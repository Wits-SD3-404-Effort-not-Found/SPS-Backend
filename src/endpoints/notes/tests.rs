use rocket::http::Status;

use crate::rocket;
use crate::tests::CLIENT;

#[test]
fn test_fetch_protocols_none_ok() {
    let client_binding = crate::tests::CLIENT.lock().unwrap();
    let response = client_binding.get(uri!(super::fetch_protocols)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_some());
}

#[test]
fn test_fetch_public_notes_none_ok() {
    let client_binding = CLIENT.lock().unwrap();
    let response = client_binding.get(uri!(super::fetch_public_notes)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_some());
}

#[test]
fn test_fetch_notes_existing_account_with_notes_ok() {
    let client_binding = CLIENT.lock().unwrap();
    let response = client_binding.get(uri!(super::fetch_notes(1))).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_some());
}

#[test]
fn test_fetch_notes_non_existing_account_not_found() {
    let client_binding = CLIENT.lock().unwrap();
    let response = client_binding.get(uri!(super::fetch_notes(0))).dispatch();
    assert_eq!(response.status(), Status::NotFound);
    assert!(response.body().is_some());
}

#[test]
fn test_fetch_notes_existing_account_no_notes_not_found() {
    let client_binding = CLIENT.lock().unwrap();
    let response = client_binding.get(uri!(super::fetch_notes(2))).dispatch();
    assert_eq!(response.status(), Status::NotFound);
    assert!(response.body().is_some());
}

#[test]
fn test_add_note_existing_account_ok() {
    let client_binding = CLIENT.lock().unwrap();

    let req_body = super::note_api::NewNote {
        account_id: 1,
        note_title: "Rocket Test Note".to_string(),
        note_content: "".to_string(),
        note_public: false,
    };

    let response = client_binding
        .post(uri!(super::add_note))
        .body(serde_json::to_string(&req_body).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_none());
}

#[test]
fn test_add_note_non_existing_account_not_found() {
    let client_binding = CLIENT.lock().unwrap();

    let req_body = super::note_api::NewNote {
        account_id: 0,
        note_title: "Rocket Test Note".to_string(),
        note_content: "".to_string(),
        note_public: false,
    };

    let response = client_binding
        .post(uri!(super::add_note))
        .body(serde_json::to_string(&req_body).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert!(response.body().is_some());
}

#[test]
fn test_update_note_existing_note_ok() {
    let client_binding = CLIENT.lock().unwrap();

    let req_body = super::note_api::UpdateNote {
        note_content: "Today we covered respiratory physiology. Here are the key points:\n\n* Oxygen and carbon dioxide exchange occurs in the alveoli of the lungs.\n* The respiratory system is controlled by the medulla oblongata in the brainstem.\n* The diaphragm and intercostal muscles are responsible for breathing.".to_string(),
        note_id: 1,
        note_title: "Lecture Notes".to_string(),
        note_public: false,
    };

    let response = client_binding
        .put(uri!(super::update_note))
        .body(serde_json::to_string(&req_body).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_none());
}

#[test]
fn test_update_note_nonexisting_note_not_found() {
    let client_binding = CLIENT.lock().unwrap();

    let req_body = super::note_api::UpdateNote {
        note_content: "Today we covered respiratory physiology. Here are the key points:\n\n* Oxygen and carbon dioxide exchange occurs in the alveoli of the lungs.\n* The respiratory system is controlled by the medulla oblongata in the brainstem.\n* The diaphragm and intercostal muscles are responsible for breathing.".to_string(),
        note_id: 0,
        note_title: "Lecture Notes".to_string(),
        note_public: false,
    };

    let response = client_binding
        .put(uri!(super::update_note))
        .body(serde_json::to_string(&req_body).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert!(response.body().is_some());
}
