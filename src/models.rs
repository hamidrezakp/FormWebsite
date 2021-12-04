use super::errors::*;
use super::schema::*;
use super::website::Db;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::Insertable;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

type Toman = i32;

#[derive(Debug, Queryable, Serialize, Deserialize, Insertable, Clone)]
#[table_name = "users"]
pub struct User {
    id: Uuid,
    username: String,
    first_name: String,
    last_name: String,
    password_hash: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewUser {
    username: String,
    first_name: String,
    last_name: String,
    password_hash: String,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Insertable, Clone)]
#[table_name = "cases"]
pub struct Case {
    id: Uuid,
    active: bool,
    registration_date: NaiveDateTime,
    editor: Uuid,
    address: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewCase {
    address: Option<String>,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Insertable, Clone)]
#[table_name = "persons"]
pub struct Person {
    id: Uuid,
    first_name: String,
    last_name: String,
    father_name: String,
    birthday: NaiveDateTime,
    national_number: String,
    phone_number: String,
    case_id: Uuid,
    is_leader: bool,
    education_field: Option<String>,
    education_location: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewPerson {
    first_name: String,
    last_name: String,
    father_name: String,
    birthday: NaiveDateTime,
    national_number: String,
    phone_number: String,
    case_id: Uuid,
    is_leader: bool,
    education_field: Option<String>,
    education_location: Option<String>,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Insertable, Clone)]
#[table_name = "person_jobs"]
pub struct PersonJob {
    id: Uuid,
    person_id: Uuid,
    title: String,
    income: Option<Toman>,
    location: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewPersonJob {
    person_id: Uuid,
    title: String,
    income: Option<i32>,
    location: Option<String>,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Insertable, Clone)]
#[table_name = "person_skills"]
pub struct PersonSkill {
    id: Uuid,
    person_id: Uuid,
    skill: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewPersonSkill {
    person_id: Uuid,
    skill: String,
}

impl User {
    pub fn new(entity: NewUser) -> Self {
        Self {
            id: Uuid::from_u128(rand::random()),
            username: entity.username,
            first_name: entity.first_name,
            last_name: entity.last_name,
            password_hash: entity.password_hash,
        }
    }

    pub async fn insert(conn: &Db, entity: Self) -> Result<()> {
        let result = conn
            .run(move |c| {
                diesel::insert_into(users::table)
                    .values(entity)
                    .returning(users::id)
                    .get_results::<Uuid>(c)
            })
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn all(conn: &Db) -> Result<Vec<User>> {
        conn.run(|c| users::dsl::users.order(users::id.desc()).load::<User>(c))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))
    }

    pub async fn get(conn: &Db, id: Uuid) -> Result<Option<User>> {
        let result = conn
            .run(move |c| users::dsl::users.find(id).get_result::<User>(c))
            .await;
        match result {
            Ok(r) => Ok(Some(r)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }
}

impl Case {
    pub fn new(entity: NewCase, editor_id: Uuid) -> Self {
        Self {
            id: Uuid::from_u128(rand::random()),
            active: true,
            registration_date: Utc::now().naive_utc(),
            editor: editor_id,
            address: entity.address,
        }
    }

    pub async fn insert(conn: &Db, entity: Self) -> Result<()> {
        let result = conn
            .run(move |c| {
                diesel::insert_into(cases::table)
                    .values(entity)
                    .returning(cases::id)
                    .get_results::<Uuid>(c)
            })
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn all(conn: &Db) -> Result<Vec<Case>> {
        conn.run(|c| cases::dsl::cases.order(cases::id.desc()).load::<Case>(c))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))
    }

    pub async fn get(conn: &Db, id: Uuid) -> Result<Option<Case>> {
        let result = conn
            .run(move |c| cases::dsl::cases.find(id).get_result::<Case>(c))
            .await;
        match result {
            Ok(r) => Ok(Some(r)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }
}

impl Person {
    pub fn new(entity: NewPerson) -> Self {
        Self {
            id: Uuid::from_u128(rand::random()),
            first_name: entity.first_name,
            last_name: entity.last_name,
            father_name: entity.father_name,
            birthday: entity.birthday,
            national_number: entity.national_number,
            phone_number: entity.phone_number,
            case_id: entity.case_id,
            is_leader: entity.is_leader,
            education_field: entity.education_field,
            education_location: entity.education_location,
        }
    }

    pub async fn insert(conn: &Db, entity: Self) -> Result<()> {
        let result = conn
            .run(move |c| {
                diesel::insert_into(persons::table)
                    .values(entity)
                    .returning(persons::id)
                    .get_results::<Uuid>(c)
            })
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn all(conn: &Db) -> Result<Vec<Person>> {
        conn.run(|c| {
            persons::dsl::persons
                .order(persons::id.desc())
                .load::<Person>(c)
        })
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))
    }

    pub async fn get(conn: &Db, id: Uuid) -> Result<Option<Person>> {
        let result = conn
            .run(move |c| persons::dsl::persons.find(id).get_result::<Person>(c))
            .await;
        match result {
            Ok(r) => Ok(Some(r)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }
}

impl PersonJob {
    pub fn new(entity: NewPersonJob) -> Self {
        Self {
            id: Uuid::from_u128(rand::random()),
            person_id: entity.person_id,
            title: entity.title,
            income: entity.income,
            location: entity.location,
        }
    }

    pub async fn insert(conn: &Db, entity: Self) -> Result<()> {
        let result = conn
            .run(move |c| {
                diesel::insert_into(person_jobs::table)
                    .values(entity)
                    .returning(person_jobs::id)
                    .get_results::<Uuid>(c)
            })
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn all(conn: &Db) -> Result<Vec<PersonJob>> {
        conn.run(|c| {
            person_jobs::dsl::person_jobs
                .order(person_jobs::id.desc())
                .load::<PersonJob>(c)
        })
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))
    }

    pub async fn get(conn: &Db, id: Uuid) -> Result<Option<PersonJob>> {
        let result = conn
            .run(move |c| {
                person_jobs::dsl::person_jobs
                    .find(id)
                    .get_result::<PersonJob>(c)
            })
            .await;
        match result {
            Ok(r) => Ok(Some(r)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }
}

impl PersonSkill {
    pub fn new(entity: NewPersonSkill) -> Self {
        Self {
            id: Uuid::from_u128(rand::random()),
            person_id: entity.person_id,
            skill: entity.skill,
        }
    }

    pub async fn insert(conn: &Db, entity: Self) -> Result<()> {
        let result = conn
            .run(move |c| {
                diesel::insert_into(person_skills::table)
                    .values(entity)
                    .returning(person_skills::id)
                    .get_results::<Uuid>(c)
            })
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn all(conn: &Db) -> Result<Vec<PersonSkill>> {
        conn.run(|c| {
            person_skills::dsl::person_skills
                .order(person_skills::id.desc())
                .load::<PersonSkill>(c)
        })
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))
    }

    pub async fn get(conn: &Db, id: Uuid) -> Result<Option<PersonSkill>> {
        let result = conn
            .run(move |c| {
                person_skills::dsl::person_skills
                    .find(id)
                    .get_result::<PersonSkill>(c)
            })
            .await;
        match result {
            Ok(r) => Ok(Some(r)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }
}
