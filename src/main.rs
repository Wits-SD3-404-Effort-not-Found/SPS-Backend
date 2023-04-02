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

#[launch]
fn rocket() -> _ {

    // If the systemd service is setup, write to the journal
    if connected_to_journal() {
        init_with_extra_fields(vec![("CARGO_VERSION", env!("CARGO_PKG_VERSION"))]).unwrap();
    } else {
        // Else just use stdout to write logs
        SimpleLogger::new().init().unwrap()
    }

    rocket::build()
        .mount("/", routes![
            endpoints::index
        ])
        .attach(db::SPS::init())
}
