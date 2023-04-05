use rocket::response::Responder;

// Custom Rocket Error handler
// Mostly for future proofing convenience atm
#[derive(Responder, Debug)]
pub enum ApiErrors {
    #[response(status = 401)]
    Unauth(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 500)]
    InternalError(String)
}

pub type ApiResult<T> = Result<T, ApiErrors>;
