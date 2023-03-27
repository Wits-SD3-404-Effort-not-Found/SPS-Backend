use super::super::rocket;
use rocket::local::blocking::Client;
use rocket::http::Status;

#[test]
fn test_index() {
    let client = Client::tracked(rocket()).expect("Valid Rocket Instance");

    let ok_response = client.get(uri!(super::index)).dispatch();
    assert_eq!(ok_response.status(), Status::Ok);
    assert_eq!(ok_response.into_string(), Some("Wits Student Placement System API".into()));
}

