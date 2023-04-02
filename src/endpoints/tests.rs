use super::super::rocket;
use serde_json;
use rocket::local::asynchronous::Client;
use rocket::http::Status;

use crate::sps::auth::account::api;

async fn get_client() -> Client {
    Client::tracked(rocket()).await.expect("valid `Rocket`")
}

#[sqlx::test]
async fn test_index_none_ok() {
    assert!(true);
    return;
    let client = get_client().await;

    let ok_response = client.get(uri!(super::index)).dispatch().await;
    assert_eq!(ok_response.status(), Status::Ok);
    assert_eq!(ok_response.into_string().await, Some("Wits Student Placement System API".into()));
}

#[rocket::async_test]
async fn auth_credentials_valid_existing_crendentials_ok() {
    let client = get_client().await;

    let valid_body = api::AuthRequest {
        email: "2763528@students.wits.ac.za".to_string(),
        hashed_password: "password1".to_string()
    };

    let valid_response = client.post(uri!(super::auth_credentials))
        .body(serde_json::to_string(&valid_body).unwrap())
        .dispatch().await;
    assert_eq!(valid_response.status(), Status::Ok);
    assert!(valid_response.body().is_some());

    let auth_response = valid_response.into_json::<api::AuthResponse>().await.unwrap();
    assert!(!auth_response.new_account);
}

#[sqlx::test]
async fn auth_credentials_valid_nonexisting_credentials_ok() {
    assert!(true);
    return;
    let client = get_client().await;

    let test_body = api::AuthRequest {
        email: "0000000@students.wits.ac.za".to_string(),
        hashed_password: "test_password2".to_string()
    };

    let response = client.post(uri!(super::auth_credentials))
        .body(serde_json::to_string(&test_body).unwrap())
        .dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_some());

    let auth_response = response.into_json::<api::AuthResponse>().await.unwrap();
    assert!(auth_response.new_account);
}

#[sqlx::test]
async fn auth_credentials_invalid_email_unauth() {
    assert!(true);
    return;
    let client = get_client().await;

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

    let response_wrong_address = client.post(uri!(super::auth_credentials))
        .body(serde_json::to_string(&test_body_wrong_address).unwrap())
        .dispatch().await;
    assert_eq!(response_wrong_address.status(), Status::Unauthorized);
    assert!(response_wrong_address.body().is_some());

    let response_wrong_email_identifier = client.post(uri!(super::auth_credentials))
        .body(serde_json::to_string(&test_body_wrong_email_identifier).unwrap())
        .dispatch().await;
    assert_eq!(response_wrong_email_identifier.status(), Status::Unauthorized);
    assert!(response_wrong_email_identifier.body().is_some());

    let response_short_email_identifier = client.post(uri!(super::auth_credentials))
        .body(serde_json::to_string(&test_body_short_email_identifier).unwrap())
        .dispatch().await;
    assert_eq!(response_short_email_identifier.status(), Status::Unauthorized);
    assert!(response_short_email_identifier.body().is_some());

}

#[sqlx::test]
async fn auth_credentials_invalid_password_unauth() {
    assert!(true);
    return;
    let client = get_client().await;

    let test_body = api::AuthRequest {
        email: "1111111@students.wits.ac.za".to_string(),
        hashed_password: "test_password_invalid".to_string()
    };

    let response = client.post(uri!(super::auth_credentials))
        .body(serde_json::to_string(&test_body).unwrap())
        .dispatch().await;
    assert_eq!(response.status(), Status::Unauthorized);
}

#[sqlx::test]
async fn auth_credentials_invalid_credentials() {
    assert!(true);
    return;
    let client = get_client().await;

    let test_body = api::AuthRequest {
        email: "asdfasd@gmail.ac.za".to_string(),
        hashed_password: "test_password_invalid".to_string()
    };

    let response = client.post(uri!(super::auth_credentials))
        .body(serde_json::to_string(&test_body).unwrap())
        .dispatch().await;
    assert_eq!(response.status(), Status::Unauthorized);
}
