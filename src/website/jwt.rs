use crate::{errors::Errors, service_options::ServiceOptions};
use chrono::{Duration, NaiveDateTime};
use jsonwebtoken::{decode, encode, errors::ErrorKind, Algorithm, EncodingKey, Header, Validation};
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
    serde::{Deserialize, Serialize},
};
use uuid::Uuid;

const TOKEN_SCHEME: &str = "Bearer";

pub struct T {}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
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

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
    pub user_id: Uuid,
    pub role: Role,
}

impl Claims {
    pub fn new(
        user_id: Uuid,
        role: Role,
        subject: String,
        created_at: NaiveDateTime,
        validity_period: Duration,
    ) -> Self {
        let expires_at = created_at + validity_period;

        Self {
            user_id,
            role,
            sub: subject,
            iat: created_at.timestamp(),
            exp: expires_at.timestamp(),
        }
    }

    pub fn to_jwt(&self, encoding_key: &EncodingKey) -> Result<String, Errors> {
        let header = Header::new(Algorithm::HS256);
        encode(&header, self, encoding_key)
            .map_err(|_| Errors::InternalError("error in creating jwt token".to_string()))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = Errors;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get_one("authorization") {
            None => Outcome::Failure((
                Status::Unauthorized,
                Errors::BadRequest("Unauthorized".to_string()),
            )),
            Some(s) => {
                if let Some(TOKEN_SCHEME) = s.get(0..TOKEN_SCHEME.len()) {
                    let s = &s[TOKEN_SCHEME.len() + 1..];

                    let service_options = request
                        .guard::<ServiceOptions>()
                        .await
                        .expect("service options is not setup");
                    let token = decode::<Claims>(
                        s,
                        &service_options.jwt_keys.decoding_key,
                        &Validation::new(Algorithm::HS256),
                    );

                    let token = match token {
                        Err(e) if e.kind() == &ErrorKind::ExpiredSignature => {
                            return Outcome::Failure((
                                Status::Unauthorized,
                                Errors::BadRequest("token expired".to_string()),
                            ))
                        }
                        Err(_) => {
                            return Outcome::Failure((
                                Status::Unauthorized,
                                Errors::BadRequest("invalid jwt token".to_string()),
                            ));
                        }
                        Ok(token) => token,
                    };

                    Outcome::Success(token.claims)
                } else {
                    return Outcome::Failure((
                        Status::Unauthorized,
                        Errors::BadRequest("invalid jwt token".to_string()),
                    ));
                }
            }
        }
    }
}

pub struct IsAdmin(pub Claims);
pub struct IsEditor(pub Claims);
pub struct IsUser(pub Claims);
pub struct IsLoggedIn(pub Claims);

pub struct HasAdminPermissions(pub Claims);
pub struct HasEditorPermissions(pub Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for IsAdmin {
    type Error = Errors;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.guard::<Claims>().await {
            Outcome::Success(token) if token.role == Role::Admin => {
                Outcome::Success(IsAdmin(token))
            }
            Outcome::Failure(e) => Outcome::Failure(e),
            Outcome::Success(token) => {
                println!("{:?}", token);
                Outcome::Forward(())
            }
            e => {
                println!("{:?}", e);
                Outcome::Forward(())
            }
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for IsEditor {
    type Error = Errors;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.guard::<Claims>().await {
            Outcome::Success(token) if token.role == Role::Editor => {
                Outcome::Success(IsEditor(token))
            }
            Outcome::Failure(e) => Outcome::Failure(e),
            _ => Outcome::Forward(()),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for IsUser {
    type Error = Errors;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.guard::<Claims>().await {
            Outcome::Success(token) if token.role == Role::User => Outcome::Success(IsUser(token)),
            Outcome::Failure(e) => Outcome::Failure(e),
            _ => Outcome::Forward(()),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for IsLoggedIn {
    type Error = Errors;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.guard::<Claims>().await {
            Outcome::Success(token) => Outcome::Success(IsLoggedIn(token)),
            Outcome::Failure(e) => Outcome::Failure(e),
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
            Outcome::Failure(e) => Outcome::Failure(e),
            _ => Outcome::Forward(()),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HasEditorPermissions {
    type Error = Errors;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.guard::<Claims>().await {
            Outcome::Success(token) => {
                if token.role == Role::Admin || token.role == Role::Editor {
                    Outcome::Success(HasEditorPermissions(token))
                } else {
                    Outcome::Forward(())
                }
            }
            Outcome::Failure(e) => Outcome::Failure(e),
            _ => Outcome::Forward(()),
        }
    }
}
