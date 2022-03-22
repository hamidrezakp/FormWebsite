use crate::errors::*;
use crate::schema::*;
use crate::user_token_service::Token;
use crate::website::Db;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::Insertable;

use rocket::request;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::Request;
use uuid::Uuid;

pub struct T {
    db_pool: Db,
}

#[derive(Debug, Queryable, Insertable, Clone, Identifiable, AsChangeset)]
#[changeset_options(treat_none_as_null = "true")]
pub struct UserToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub subject: String,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub payload: Option<String>,
}

impl UserToken {
    pub fn of_token(id: Uuid, entity: Token) -> Self {
        UserToken {
            id,
            user_id: entity.user_id,
            subject: entity.subject,
            token: entity.token,
            created_at: entity.created_at,
            expires_at: entity.expires_at,
            payload: entity.payload,
        }
    }

    pub fn of_new_token(entity: Token) -> Self {
        UserToken {
            id: Uuid::from_u128(rand::random()),
            user_id: entity.user_id,
            subject: entity.subject,
            token: entity.token,
            created_at: entity.created_at,
            expires_at: entity.expires_at,
            payload: entity.payload,
        }
    }

    pub fn into_token(self) -> (Uuid, Token) {
        (
            self.id,
            Token {
                user_id: self.user_id,
                subject: self.subject,
                token: self.token,
                created_at: self.created_at,
                expires_at: self.expires_at,
                payload: self.payload,
            },
        )
    }
}

impl T {
    pub fn create(db_pool: Db) -> Self {
        Self { db_pool }
    }

    pub async fn insert(&self, entity: Token) -> Result<Uuid> {
        use crate::schema::user_tokens::dsl::*;

        let result = self
            .db_pool
            .run(move |c| {
                diesel::insert_into(user_tokens)
                    .values(UserToken::of_new_token(entity))
                    .returning(id)
                    .get_result::<Uuid>(c)
            })
            .await;

        match result {
            Ok(i) => Ok(i),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    #[allow(dead_code)]
    pub async fn update(&self, p_id: Uuid, entity: Token) -> Result<()> {
        use self::user_tokens::dsl::*;

        let result = self
            .db_pool
            .run(move |c| {
                diesel::update(user_tokens)
                    .filter(id.eq(p_id))
                    .set(UserToken::of_token(p_id, entity))
                    .execute(c)
            })
            .await;

        match result {
            Ok(count) if count == 1 => Ok(()),
            Ok(_) => Err(Errors::BadRequest("id not found".to_owned())),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    #[allow(dead_code)]
    pub async fn delete(&self, p_id: Uuid) -> Result<()> {
        use self::user_tokens::dsl::*;

        let count = self
            .db_pool
            .run(move |c| diesel::delete(user_tokens.filter(id.eq(p_id))).execute(c))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        match count {
            0 => Err(Errors::DatabaseError("id not found".to_owned())),
            _ => Ok(()),
        }
    }

    #[allow(dead_code)]
    pub async fn get(&self, p_id: Uuid) -> Result<Option<(Uuid, Token)>> {
        use self::user_tokens::dsl::*;

        let result = self
            .db_pool
            .run(move |c| user_tokens.find(p_id).get_result::<UserToken>(c))
            .await;
        match result {
            Ok(r) => Ok(Some(UserToken::into_token(r))),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn get_by_subject_and_token(
        &self,
        p_subject: String,
        p_token: String,
    ) -> Result<Option<(Uuid, Token)>> {
        use self::user_tokens::dsl::*;

        let result = self
            .db_pool
            .run(move |c| {
                user_tokens
                    .filter(subject.eq(p_subject))
                    .filter(token.eq(p_token))
                    .first::<UserToken>(c)
            })
            .await;
        match result {
            Ok(r) => Ok(Some(UserToken::into_token(r))),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn revoke_by_user_and_subject(
        &self,
        p_user_id: Uuid,
        p_subject: String,
    ) -> Result<()> {
        use self::user_tokens::dsl::*;

        let count = self
            .db_pool
            .run(move |c| {
                diesel::delete(user_tokens)
                    .filter(subject.eq(p_subject))
                    .filter(user_id.eq(p_user_id))
                    .execute(c)
            })
            .await;
        match count {
            Ok(_) | Err(diesel::result::Error::NotFound) => Ok(()),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for T {
    type Error = Errors;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let db = request
            .guard::<Db>()
            .await
            .expect("create user_token_repository");
        Outcome::Success(T::create(db))
    }
}
