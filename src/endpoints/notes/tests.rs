use rocket::http::Status;

use crate::rocket;

#[test]
fn test_fetch_protocols_none_ok() {
    let client_binding = crate::tests::CLIENT.blocking_lock();
    let response = client_binding.get(uri!(super::fetch_protocols)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_some());
}
