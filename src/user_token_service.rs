use crate::{
    errors::{self, Errors},
    repository::user_token_repository,
};
use chrono::{Duration, NaiveDateTime, Utc};
use rocket::{
    request::{self, FromRequest, Outcome},
    Request,
};
use uuid::Uuid;

pub struct T {
    repo: user_token_repository::T,
}

#[derive(Clone)]
pub struct Token {
    pub user_id: Uuid,
    pub subject: String,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub payload: Option<String>,
}

impl T {
    pub fn make(repo: user_token_repository::T) -> Self {
        Self { repo }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        token: String,
        subject: String,
        validity_period: Duration,
        payload: Option<String>,
    ) -> errors::Result<Token> {
        let now = Utc::now().naive_utc();
        let token = Token {
            user_id,
            subject,
            token,
            created_at: now,
            expires_at: now + validity_period,
            payload,
        };

        let _ = self.repo.insert(token.clone()).await?;
        Ok(token)
    }

    pub async fn get(&self, subject: String, token: String) -> errors::Result<Option<Token>> {
        match self.repo.get_by_subject_and_token(subject, token).await? {
            None => Ok(None),
            Some((_, token)) => Ok(Some(token)),
        }
    }

    pub async fn revoke(&self, user_id: Uuid, subject: String) -> errors::Result<()> {
        self.repo.revoke_by_user_and_subject(user_id, subject).await
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for T {
    type Error = Errors;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let repo = request
            .guard::<user_token_repository::T>()
            .await
            .expect("create user_token_service");
        Outcome::Success(T::make(repo))
    }
}
