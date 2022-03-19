use crate::{errors::Errors, service_options::ServiceOptions};
use jsonwebtoken::{decode, encode, Algorithm, EncodingKey, Header, Validation};
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
    serde::{Deserialize, Serialize},
};
use uuid::Uuid;

#[derive(Deserialize, Serialize, PartialEq)]
pub enum Role {
    Admin,
    Editor,
    User,
}

impl From<i32> for Role {
    fn from(number: i32) -> Self {
        match number {
            0 => Role::Admin,
            1 => Role::Editor,
            2 => Role::User,
            _ => panic!("invalid user role number {}", number),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct UserToken {
    pub user_id: Uuid,
    pub role: Role,
}

impl UserToken {
    pub fn new(user_id: Uuid, role: Role) -> Self {
        Self { user_id, role }
    }

    pub fn to_jwt(&self, encoding_key: &EncodingKey) -> Result<String, Errors> {
        let header = Header::new(Algorithm::HS256);
        encode(&header, self, encoding_key)
            .map_err(|_| Errors::InternalError("error in creating jwt token".to_string()))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserToken {
    type Error = Errors;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get_one("authorization") {
            None => Outcome::Failure((
                Status::Unauthorized,
                Errors::BadRequest("Unauthorized".to_string()),
            )),
            Some(s) => {
                let service_options = request
                    .rocket()
                    .state::<ServiceOptions>()
                    .expect("service options is not setup");
                let token = decode::<UserToken>(
                    s,
                    &service_options.jwt_decoding_key,
                    &Validation::new(Algorithm::HS256),
                );

                let token = match token {
                    Err(_) => {
                        return Outcome::Failure((
                            Status::Unauthorized,
                            Errors::BadRequest("invalid jwt token".to_string()),
                        ))
                    }
                    Ok(token) => token,
                };

                Outcome::Success(token.claims)
            }
        }
    }
}

pub struct IsAdmin(pub UserToken);
pub struct IsEditor(pub UserToken);
pub struct IsUser(pub UserToken);
pub struct IsLoggedIn(pub UserToken);

pub struct HasAdminPermissions(pub UserToken);
pub struct HasEditorPermissions(pub UserToken);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for IsAdmin {
    type Error = Errors;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.guard::<UserToken>().await {
            Outcome::Success(token) if token.role == Role::Admin => {
                Outcome::Success(IsAdmin(token))
            }
            _ => Outcome::Forward(()),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for IsEditor {
    type Error = Errors;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.guard::<UserToken>().await {
            Outcome::Success(token) if token.role == Role::Editor => {
                Outcome::Success(IsEditor(token))
            }
            _ => Outcome::Forward(()),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for IsUser {
    type Error = Errors;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.guard::<UserToken>().await {
            Outcome::Success(token) if token.role == Role::User => Outcome::Success(IsUser(token)),
            _ => Outcome::Forward(()),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for IsLoggedIn {
    type Error = Errors;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.guard::<UserToken>().await {
            Outcome::Success(token) => Outcome::Success(IsLoggedIn(token)),
            _ => Outcome::Forward(()),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HasAdminPermissions {
    type Error = Errors;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.guard::<IsAdmin>().await {
            Outcome::Success(IsAdmin(token)) => Outcome::Success(HasAdminPermissions(token)),
            _ => Outcome::Forward(()),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HasEditorPermissions {
    type Error = Errors;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.guard::<UserToken>().await {
            Outcome::Success(token) => {
                if token.role == Role::Admin || token.role == Role::Editor {
                    {
                        Outcome::Success(HasEditorPermissions(token))
                    }
                } else {
                    Outcome::Forward(())
                }
            }
            _ => Outcome::Forward(()),
        }
    }
}
