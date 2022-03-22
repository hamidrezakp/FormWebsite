use jsonwebtoken::{DecodingKey, EncodingKey};
use rocket::{
    figment::Figment,
    http::Status,
    request::{self, FromRequest, Outcome},
    Request,
};

use crate::errors::{self, Errors};

pub struct JWTKeys {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}

pub struct ServiceOptions {
    pub jwt_keys: JWTKeys,
}

impl ServiceOptions {
    pub fn create(figment: &Figment) -> errors::Result<Self> {
        let secret_key: String = match figment.extract_inner("secret_key") {
            Err(_) => {
                return Err(errors::Errors::InternalError(
                    "cannot find secret_key".into(),
                ));
            }
            Ok(s) => s,
        };

        let opts = Self {
            jwt_keys: JWTKeys {
                encoding_key: EncodingKey::from_secret(secret_key.as_bytes()),
                decoding_key: DecodingKey::from_secret(secret_key.as_bytes()),
            },
        };
        Ok(opts)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ServiceOptions {
    type Error = Errors;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let opts = ServiceOptions::create(request.rocket().figment());

        match opts {
            Err(_e) => {
                return Outcome::Failure((
                    Status::InternalServerError,
                    Errors::InternalError("cannot create serviceOptions".to_string()),
                ));
            }
            Ok(opts) => Outcome::Success(opts),
        }
    }
}
