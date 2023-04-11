use rocket::local::blocking::Client;
use rocket::http::Status;
use lazy_static::lazy_static;
use tokio::sync::Mutex;

use crate::rocket;

lazy_static! {
    static ref CLIENT: Mutex<Client> = Mutex::new(
        Client::tracked(rocket()).expect("valid rocket instance") 
    );
}

#[test]
fn test_fetch_protocols_none_ok() {
    let client_binding = CLIENT.blocking_lock();
    let response = client_binding.get(uri!(super::fetch_protocols)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_some());
}
