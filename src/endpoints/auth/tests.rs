use rocket::http::Status;

use crate::rocket;

#[test]
fn test_validate_email_valid_email_true() {
    let email = "0000000@students.wits.ac.za".to_string();
    let result = match super::validate_email(&email) {
        Ok(_) => true,
        Err(_) => false,
    };
    assert!(result);
}

#[test]
fn test_validate_email_invalid_email_false() {
    let email_1 = "asdfasd@students.wits.ac.za".to_string();
    let result_1 = match super::validate_email(&email_1) {
        Ok(_) => true,
        Err(_) => false,
    };
    assert!(!result_1);

    let email_2 = "asdfasd@gmail.com".to_string();
    let result_2 = match super::validate_email(&email_2) {
        Ok(_) => true,
        Err(_) => false,
    };
    assert!(!result_2);

    let email_3 = "12345@students.wits.ac.za".to_string();
    let result_3 = match super::validate_email(&email_3) {
        Ok(_) => true,
        Err(_) => false,
    };
    assert!(!result_3);
}

#[test]
fn test_auth_credentials_correct_credentials_ok() {
    let client_binding = crate::tests::CLIENT.lock().unwrap();

    let valid_body = super::credentials::CredentialRequest {
        email: "2763528@students.wits.ac.za".to_string(),
        hashed_password: "0b14d501a594442a01c6859541bcb3e8164d183d32937b851835442f69d5c94e"
            .to_string(),
    };

    let response = client_binding
        .post(uri!(super::auth_credentials))
        .body(serde_json::to_string(&valid_body).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_some());
}

#[test]
fn test_auth_credentials_incorrect_credentials_unauth() {
    let client_binding = crate::tests::CLIENT.lock().unwrap();

    let valid_body = super::credentials::CredentialRequest {
        email: "0000000@gmail.wits.ac.za".to_string(),
        hashed_password: "password2".to_string(),
    };

    let response = client_binding
        .post(uri!(super::auth_credentials))
        .body(serde_json::to_string(&valid_body).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
    assert!(response.body().is_some());
}

#[test]
fn test_auth_security_questions_valid_email_ok() {
    let client_binding = crate::tests::CLIENT.lock().unwrap();
    let invalid_email_body = super::security_questions::SecurityQuestionsRequest {
        email: "2763528@students.wits.ac.za".to_string()
    };


    let response = client_binding.post(uri!(super::auth_security_questions))
        .body(serde_json::to_string(&invalid_email_body).unwrap())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_some());
}

#[test]
fn test_auth_security_questions_invalid_email_unauth() {
    let client_binding = crate::tests::CLIENT.lock().unwrap();
    let invalid_email_body = super::security_questions::SecurityQuestionsRequest {
        email: "0000000@gmail.wits.ac.za".to_string()
    };

    let response = client_binding.post(uri!(super::auth_security_questions))
        .body(serde_json::to_string(&invalid_email_body).unwrap())
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
    assert!(response.body().is_some());
}

#[test]
fn test_auth_session_no_token_unauth() {
    let client_binding = crate::tests::CLIENT.lock().unwrap();
    let body = super::session_token::TokenRequest {
        session_token: "asdfasdf".to_string(),
        account_id: 1
    };

    let response = client_binding.post(uri!(super::auth_session))
        .body(serde_json::to_string(&body).unwrap()).dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
    assert!(response.body().is_some());
}