#[cfg(test)] mod tests;

#[get("/")]
pub fn index() -> &'static str {
    log::info!("Connecting to API Index");
    "Wits Student Placement System API"
}

