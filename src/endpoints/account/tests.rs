use rocket::http::Status;
use rocket::http::hyper::Response;

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
        profile_photo: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
    };

    let response = client_binding
        .put(uri!(super::update_account))
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
        profile_photo: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
    };

    let response = client_binding
        .put(uri!(super::update_account))
        .body(serde_json::to_string(&req_body).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert!(response.body().is_some());
}

#[test]
fn test_account_reset_password_correct_answers_ok() {
    let client_binding = CLIENT.lock().unwrap();
    let req_body = super::password::NewPasswordRequest {
        account_id: 1,
        new_password:  "0b14d501a594442a01c6859541bcb3e8164d183d32937b851835442f69d5c94e".to_string(),
        questions: vec![
            super::security_question::SecurityQuestion { question_id: 1, user_answer: "a459891617d735655dcfed3e37db66fa07f0175866ebf35f9de8ccc59c0840bb".to_string() },
            super::security_question::SecurityQuestion { question_id: 2, user_answer: "b93b9776163702f1fad6cbaf815326a41b3285d0961f4e838ebdb8ad52e5f16e".to_string() },
        ],
    };

    let response = client_binding.post(uri!(super::account_reset_password))
        .body(serde_json::to_string(&req_body).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_none());
}

#[test]
fn test_account_reset_password_incorrect_answers_unauth() {
    let client_binding = CLIENT.lock().unwrap();
    let req_body = super::password::NewPasswordRequest {
        account_id: 1,
        new_password:  "0b14d501a594442a01c6859541bcb3e8164d183d32937b851835442f69d5c94e".to_string(),
        questions: vec![
            super::security_question::SecurityQuestion { question_id: 1, user_answer: "lbue".to_string() },
            super::security_question::SecurityQuestion { question_id: 2, user_answer: "green".to_string() },
        ],
    };

    let response = client_binding.post(uri!(super::account_reset_password))
        .body(serde_json::to_string(&req_body).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
    assert!(response.body().is_some());
}

#[test]
fn test_account_reset_password_no_account_not_found() {
    let client_binding = CLIENT.lock().unwrap();
    let req_body = super::password::NewPasswordRequest {
        account_id: 0,
        new_password:  "".to_string(),
        questions: vec![
            super::security_question::SecurityQuestion { question_id: 1, user_answer: "".to_string() },
            super::security_question::SecurityQuestion { question_id: 2, user_answer: "".to_string() },
        ],
    };

    let response = client_binding.post(uri!(super::account_reset_password))
        .body(serde_json::to_string(&req_body).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert!(response.body().is_some());
}

#[test]
fn test_add_questions_existing_account_ok() {
    let client_binding = CLIENT.lock().unwrap();
    let req_body = super::security_question::AddSecurityQuestion{
        account_id: 1,
        questions: vec![
            super::security_question::SecurityQuestion { question_id: 1, user_answer: "a459891617d735655dcfed3e37db66fa07f0175866ebf35f9de8ccc59c0840bb".to_string() },
            super::security_question::SecurityQuestion { question_id: 2, user_answer: "b93b9776163702f1fad6cbaf815326a41b3285d0961f4e838ebdb8ad52e5f16e".to_string() },
        ],
    };

    let response = client_binding
        .post(uri!(super::add_questions))
        .body(serde_json::to_string(&req_body).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_none());
}

#[test]
fn test_add_questions_nonexisting_account_ok() {
    let client_binding = CLIENT.lock().unwrap();
    let req_body = super::security_question::AddSecurityQuestion{
        account_id: 0,
        questions: vec![
            super::security_question::SecurityQuestion { question_id: 1, user_answer: "a459891617d735655dcfed3e37db66fa07f0175866ebf35f9de8ccc59c0840bb".to_string() },
            super::security_question::SecurityQuestion { question_id: 2, user_answer: "b93b9776163702f1fad6cbaf815326a41b3285d0961f4e838ebdb8ad52e5f16e".to_string() },
        ],
    };

    let response = client_binding
        .post(uri!(super::add_questions))
        .body(serde_json::to_string(&req_body).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert!(response.body().is_some());
}