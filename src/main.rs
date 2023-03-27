mod endpoints;

#[macro_use] extern crate rocket;

use simple_logger::SimpleLogger;
use systemd_journal_logger::{
    connected_to_journal,
    init_with_extra_fields
};

#[launch]
fn rocket() -> _ {

    if connected_to_journal() {
        init_with_extra_fields(vec![("CARGO_VERSION", env!("CARGO_PKG_VERSION"))]).unwrap();
    } else {
        SimpleLogger::new().init().unwrap()
    }

    rocket::build()
        .mount("/", routes![
            endpoints::index
        ])
}
