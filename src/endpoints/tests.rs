use super::super::rocket;
use serde_json;
use rocket::local::blocking::Client;
use rocket::http::Status;

use crate::sps::auth::account::api;

#[test]
fn test_index_none_ok() {
    let client = Client::tracked(rocket()).expect("Valid Rocket Instance");

    let ok_response = client.get(uri!(super::index)).dispatch();
    assert_eq!(ok_response.status(), Status::Ok);
    assert_eq!(ok_response.into_string(), Some("Wits Student Placement System API".into()));
}

#[test]
fn test_auth_credentials_existing_valid_credentials_ok() {
    let test_body = api::AuthRequest {
        email: "1111111@students.wits.ac.za".to_string(),
        hashed_password: "test_password1".to_string()
    };
    let client = Client::tracked(rocket()).expect("Valid Rocket Instance");

    let response = client.post(uri!(super::auth_credentials))
        .body(serde_json::to_string(&test_body).unwrap())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_some());

    let auth_response = response.into_json::<api::AuthResponse>().unwrap();
    assert!(!auth_response.new_account);
}

#[test]
fn test_auth_credentials_nonexisting_valid_credentials_ok() {
    let test_body = api::AuthRequest {
        email: "0000000@students.wits.ac.za".to_string(),
        hashed_password: "test_password2".to_string()
    };
    let client = Client::tracked(rocket()).expect("Valid Rocket Instance");

    let response = client.post(uri!(super::auth_credentials))
        .body(serde_json::to_string(&test_body).unwrap())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_some());

    let auth_response = response.into_json::<api::AuthResponse>().unwrap();
    assert!(auth_response.new_account);
}

#[test]
fn test_auth_credentials_invalid_email_unauth() {
    let test_body_wrong_address = api::AuthRequest {
        email: "1111111@gmail.com".to_string(),
        hashed_password: "test_password2".to_string()
    };

    let test_body_wrong_email_identifier = api::AuthRequest {
        email: "abcdefg@students.wits.ac.za".to_string(),
        hashed_password: "test_password2".to_string()
    };

    let test_body_short_email_identifier = api::AuthRequest {
        email: "111111@students.wits.ac.za".to_string(),
        hashed_password: "test_password2".to_string()
    };

    let client = Client::tracked(rocket()).expect("Valid Rocket Instance");

    let response_wrong_address = client.post(uri!(super::auth_credentials))
        .body(serde_json::to_string(&test_body_wrong_address).unwrap())
        .dispatch();
    assert_eq!(response_wrong_address.status(), Status::Unauthorized);
    assert!(response_wrong_address.body().is_some());

    let response_wrong_email_identifier = client.post(uri!(super::auth_credentials))
        .body(serde_json::to_string(&test_body_wrong_email_identifier).unwrap())
        .dispatch();
    assert_eq!(response_wrong_email_identifier.status(), Status::Unauthorized);
    assert!(response_wrong_email_identifier.body().is_some());

    let response_short_email_identifier = client.post(uri!(super::auth_credentials))
        .body(serde_json::to_string(&test_body_short_email_identifier).unwrap())
        .dispatch();
    assert_eq!(response_short_email_identifier.status(), Status::Unauthorized);
    assert!(response_short_email_identifier.body().is_some());
}

#[test]
fn test_auth_credentials_invalid_password_unauth() {
    let test_body = api::AuthRequest {
        email: "1111111@students.wits.ac.za".to_string(),
        hashed_password: "test_password_invalid".to_string()
    };

    let client = Client::tracked(rocket()).expect("Valid Rocket Instance");

    let response = client.post(uri!(super::auth_credentials))
        .body(serde_json::to_string(&test_body).unwrap())
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn test_auth_credentials_invalid_credentials_unauth() {
    let test_body = api::AuthRequest {
        email: "asdfasd@gmail.ac.za".to_string(),
        hashed_password: "test_password_invalid".to_string()
    };

    let client = Client::tracked(rocket()).expect("Valid Rocket Instance");

    let response = client.post(uri!(super::auth_credentials))
        .body(serde_json::to_string(&test_body).unwrap())
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}
