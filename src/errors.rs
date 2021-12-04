#[derive(Responder, Debug)]
#[response(status = 500, content_type = "json")]
pub enum Errors {
    DatabaseError(String),
}

pub type Result<T> = std::result::Result<T, Errors>;
