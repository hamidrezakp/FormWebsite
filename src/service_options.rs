use jsonwebtoken::DecodingKey;
use rocket::{Build, Rocket};

pub struct ServiceOptions {
    pub jwt_decoding_key: DecodingKey,
}

impl ServiceOptions {
    pub async fn of_rocket_ignite(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
        let figment = rocket.figment();
        let secret_key: [u8; 32] = match figment.extract_inner("secret_key") {
            Err(_) => return Err(rocket),
            Ok(s) => s,
        };

        let opts = Self {
            jwt_decoding_key: DecodingKey::from_secret(&secret_key),
        };

        Ok(rocket.manage(opts))
    }
}
