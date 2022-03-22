use super::jwt;
use super::users::models::UserInfo;
use super::Db;
use crate::errors::*;
use crate::models::*;
use crate::service_options::ServiceOptions;
use crate::user_token_service;
use chrono::Duration;
use chrono::Utc;
use rocket::serde::json::Json;
use rocket::Route;

const ACCESS_TOKEN_SUBJECT: &str = "ACCESS_TOKEN";
const ACCESS_TOKEN_DURATION_SECONDS: i64 = 300;

const REFRESH_TOKEN_SUBJECT: &str = "REFRESH_TOKEN";
const REFRESH_TOKEN_DURATION_SECONDS: i64 = 604800;

mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize)]
    pub struct LoginRequest {
        pub username: String,
        pub password: String,
    }

    #[derive(Serialize)]
    pub struct LoginResponse {
        pub access_token: String,
        pub refresh_token: String,
    }

    #[derive(Deserialize)]
    pub struct RefreshRequest {
        pub refresh_token: String,
    }
}

#[get("/user-info")]
async fn get_info(conn: Db, token: jwt::IsLoggedIn) -> Result<Option<Json<UserInfo>>> {
    let user = User::get(&conn, token.0.user_id).await?;
    Ok(user.map(UserInfo::of_user).map(Json))
}

#[post("/login", data = "<login_request>")]
async fn login(
    login_request: Json<models::LoginRequest>,
    conn: Db,
    user_token_service: user_token_service::T,
    opts: ServiceOptions,
) -> Result<Option<Json<models::LoginResponse>>> {
    use sha2::{Digest, Sha256};

    let user = match User::get_by_username(&conn, login_request.username.clone()).await? {
        None => return Ok(None),
        Some(user) => user,
    };

    let mut hasher = Sha256::new();
    hasher.update(login_request.password.as_bytes());
    hasher.update(user.password_salt.clone());

    if hasher.finalize().as_slice() != user.password_hash {
        return Err(Errors::BadRequest("Invalid Credentials".into()));
    }

    let access_token = jwt::Claims::new(
        user.id,
        user.role.into(),
        ACCESS_TOKEN_SUBJECT.into(),
        Utc::now().naive_utc(),
        Duration::seconds(ACCESS_TOKEN_DURATION_SECONDS),
    )
    .to_jwt(&opts.jwt_keys.encoding_key)?;

    let refresh_token = rand::random::<[u8; 32]>()
        .iter()
        .map(|x| format!("{:x}", x))
        .reduce(|ac, item| format!("{}{}", ac, item))
        .ok_or_else(|| Errors::InternalError("cannot create refresh token".into()))?;

    user_token_service
        .revoke(user.id, REFRESH_TOKEN_SUBJECT.into())
        .await?;

    let _ = user_token_service
        .create(
            user.id,
            refresh_token.clone(),
            REFRESH_TOKEN_SUBJECT.into(),
            Duration::seconds(REFRESH_TOKEN_DURATION_SECONDS),
            None,
        )
        .await?;

    let response = models::LoginResponse {
        access_token,
        refresh_token,
    };

    Ok(Some(Json(response)))
}

#[post("/logout")]
async fn logout(claims: jwt::IsLoggedIn, user_token_service: user_token_service::T) -> Result<()> {
    user_token_service
        .revoke(claims.0.user_id, REFRESH_TOKEN_SUBJECT.into())
        .await
}

#[post("/refresh", data = "<refresh_request>")]
async fn refresh(
    refresh_request: Json<models::RefreshRequest>,
    conn: Db,
    user_token_service: user_token_service::T,
    opts: ServiceOptions,
) -> Result<Option<Json<models::LoginResponse>>> {
    if let Some(token) = user_token_service
        .get(
            REFRESH_TOKEN_SUBJECT.into(),
            refresh_request.refresh_token.clone(),
        )
        .await?
    {
        let user = User::get(&conn, token.user_id)
            .await?
            .ok_or_else(|| Errors::InternalError("user not found".into()))?;

        let access_token = jwt::Claims::new(
            user.id,
            user.role.into(),
            ACCESS_TOKEN_SUBJECT.into(),
            Utc::now().naive_utc(),
            Duration::seconds(ACCESS_TOKEN_DURATION_SECONDS),
        )
        .to_jwt(&opts.jwt_keys.encoding_key)?;

        let refresh_token = rand::random::<[u8; 32]>()
            .iter()
            .map(|x| format!("{:x}", x))
            .reduce(|ac, item| format!("{}{}", ac, item))
            .ok_or_else(|| Errors::InternalError("cannot create refresh token".into()))?;

        user_token_service
            .revoke(user.id, REFRESH_TOKEN_SUBJECT.into())
            .await?;

        let _ = user_token_service
            .create(
                user.id,
                refresh_token.clone(),
                REFRESH_TOKEN_SUBJECT.into(),
                Duration::seconds(REFRESH_TOKEN_DURATION_SECONDS),
                None,
            )
            .await?;

        let response = models::LoginResponse {
            access_token,
            refresh_token,
        };

        Ok(Some(Json(response)))
    } else {
        Err(Errors::BadRequest("invalid refresh token".into()))
    }
}

pub fn get_routes() -> Vec<Route> {
    routes![get_info, login, refresh, logout]
}
