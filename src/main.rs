mod endpoints;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            endpoints::index
        ])
}
