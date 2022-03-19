#[derive(Responder, Debug)]
pub enum Errors {
    #[response(status = 500, content_type = "json")]
    DatabaseError(String),

    #[response(status = 400, content_type = "json")]
    BadRequest(String),

    #[response(status = 501, content_type = "json")]
    InternalError(String),
}

pub type Result<T> = std::result::Result<T, Errors>;
