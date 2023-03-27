#[cfg(test)] mod tests;

#[get("/")]
pub fn index() -> &'static str {
    "Wits Student Placement System API"
}

