mod endpoints;
mod db;

// Any errors croping up on the next 3 lines can just be ignored
#[macro_use]
extern crate rocket;
extern crate lazy_static;

use simple_logger::SimpleLogger;
use rocket_db_pools::Database;

#[cfg(test)]
pub mod tests {
    use rocket::local::blocking::Client;
    use std::sync::Mutex;
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref CLIENT: Mutex<Client> = Mutex::new(
            Client::tracked(super::rocket()).expect("valid rocket instance")
        );
    }
}

#[cfg(not(tarpaulin_include))]
#[launch]
fn rocket() -> _ {

    // Use simple logger for logging purposes
    match SimpleLogger::new().init() {
        Err(e) => println!("Logger Error: {e}"),
        _ => ()
    }

    // Rocket HTTP server creation routine
    rocket::build()
        .mount("/", routes![
            endpoints::index,
            endpoints::auth::auth_credentials,
            endpoints::auth::auth_session,
            endpoints::auth::auth_security_questions,
            endpoints::account::account_reset_password,
            endpoints::notes::fetch_protocols,
            endpoints::notes::fetch_notes,
            endpoints::notes::add_note,
            endpoints::notes::remove_note,
            endpoints::notes::update_note,
            endpoints::events::fetch_events
        ])
        .attach(db::SPS::init())
}
