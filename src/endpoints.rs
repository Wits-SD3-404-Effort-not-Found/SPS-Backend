#[cfg(test)] mod tests;

use crate::SETTINGS;

// Code Demo:
//  for defining an index
//  Accessing settings, its async but might need to switched to sync later
#[get("/")]
pub async fn index() -> &'static str {
    log::info!("Connecting to API Index");
    async {
        let settings = SETTINGS.read().await;
        log::info!("test {}", settings.get::<String>("test_value").unwrap());
    }.await;

    "Wits Student Placement System API"
}

