mod endpoints;
mod db;

// Any errors croping up on the next 3 lines can just be ignored
#[macro_use]
extern crate rocket;
extern crate lazy_static;

use simple_logger::SimpleLogger;
use systemd_journal_logger::{
    connected_to_journal,
    init_with_extra_fields
};
use config::Config;
use lazy_static::lazy_static;
use tokio::sync::RwLock;
use rocket_db_pools::Database;

// Add all our settings to a global variable
// RwLock - https://docs.rs/tokio/1.26.0/tokio/sync/struct.RwLock.html
// lazy_static - https://docs.rs/lazy_static/latest/lazy_static/
// config - https://docs.rs/config/0.13.3/config/
lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(
        Config::builder()
            .add_source(config::File::with_name("./config.toml")).build().unwrap()
    );
}

#[cfg(test)]
pub mod tests {
    use rocket::local::blocking::Client;
    use tokio::sync::Mutex;
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

    // If the systemd service is setup, write to the journal
    if connected_to_journal() {
        init_with_extra_fields(vec![("CARGO_VERSION", env!("CARGO_PKG_VERSION"))]).unwrap();
    } else {
        // Else just use stdout to write logs
        match SimpleLogger::new().init() {
            Err(e) => println!("Logger Error: {e}"),
            _ => ()
        }
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
        ])
        .attach(db::SPS::init())
}
