use super::errors::*;
use super::schema::*;
use super::website::Db;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::Insertable;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

type Toman = i32;

#[derive(
    Debug, Queryable, Serialize, Deserialize, Insertable, Clone, Identifiable, AsChangeset,
)]
#[changeset_options(treat_none_as_null = "true")]
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

#[derive(
    Debug, Queryable, Serialize, Deserialize, Insertable, Identifiable, AsChangeset, Clone,
)]
#[changeset_options(treat_none_as_null = "true")]
pub struct Case {
    id: Uuid,
    number: i32,
    active: bool,
    registration_date: NaiveDateTime,
    editor: Uuid,
    address: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewCase {
    address: Option<String>,
}

#[derive(
    Debug, Queryable, Serialize, Deserialize, Insertable, Identifiable, AsChangeset, Clone,
)]
#[changeset_options(treat_none_as_null = "true")]
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
    is_leader: bool,
    education_field: Option<String>,
    education_location: Option<String>,
}

#[derive(
    Debug, Queryable, Serialize, Deserialize, Insertable, Identifiable, AsChangeset, Clone,
)]
#[changeset_options(treat_none_as_null = "true")]
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

#[derive(
    Debug, Queryable, Serialize, Deserialize, Insertable, Identifiable, AsChangeset, Clone,
)]
#[changeset_options(treat_none_as_null = "true")]
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

#[derive(Debug, Queryable, Serialize, Deserialize, Insertable, AsChangeset, Clone)]
#[changeset_options(treat_none_as_null = "true")]
pub struct PersonRequirement {
    id: Uuid,
    person_id: Uuid,
    description: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewPersonRequirement {
    person_id: Uuid,
    description: String,
}

impl User {
    pub async fn new(conn: &Db, entity: NewUser) -> Result<Self> {
        use self::users::dsl::*;

        let mut results = conn
            .run(move |c| {
                diesel::insert_into(users)
                    .values((
                        id.eq(Uuid::from_u128(rand::random())),
                        username.eq(entity.username),
                        first_name.eq(entity.first_name),
                        last_name.eq(entity.last_name),
                        password_hash.eq(entity.password_hash),
                    ))
                    .get_results(c)
            })
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        match results.pop() {
            Some(entity) => Ok(entity),
            None => Err(Errors::DatabaseError(
                "error while inserting new item to database".to_string(),
            )),
        }
    }

    pub async fn insert(conn: &Db, entity: Self) -> Result<()> {
        use self::users::dsl::*;

        let result = conn
            .run(move |c| {
                diesel::insert_into(users)
                    .values(entity)
                    .returning(id)
                    .get_results::<Uuid>(c)
            })
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn update(conn: &Db, entity: Self) -> Result<()> {
        use self::users::dsl::*;

        let result = conn
            .run(move |c| diesel::update(users).set(&entity).execute(c))
            .await;

        match result {
            Ok(count) if count == 1 => Ok(()),
            Ok(_) => Err(Errors::DatabaseError(
                "Uuid is wrong, can not update".to_owned(),
            )),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn all(conn: &Db) -> Result<Vec<User>> {
        use self::users::dsl::*;

        conn.run(|c| users.order(id.desc()).load::<User>(c))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))
    }

    pub async fn get(conn: &Db, p_id: Uuid) -> Result<Option<User>> {
        use self::users::dsl::*;

        let result = conn
            .run(move |c| users.find(p_id).get_result::<User>(c))
            .await;
        match result {
            Ok(r) => Ok(Some(r)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn delete(conn: &Db, p_id: Uuid) -> Result<()> {
        use self::users::dsl::*;

        let count = conn
            .run(move |c| diesel::delete(users.filter(id.eq(p_id))).execute(c))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        match count {
            0 => Err(Errors::DatabaseError("id not found".to_owned())),
            _ => Ok(()),
        }
    }
}

impl Case {
    pub async fn new(conn: &Db, entity: NewCase, editor_id: Uuid) -> Result<Self> {
        use self::cases::dsl::*;

        let mut results = conn
            .run(move |c| {
                diesel::insert_into(cases)
                    .values((
                        id.eq(Uuid::from_u128(rand::random())),
                        active.eq(true),
                        registration_date.eq(Utc::now().naive_utc()),
                        editor.eq(editor_id),
                        address.eq(entity.address),
                    ))
                    .get_results(c)
            })
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        match results.pop() {
            Some(entity) => Ok(entity),
            None => Err(Errors::DatabaseError(
                "error while inserting new item to database".to_string(),
            )),
        }
    }

    pub async fn insert<'a>(conn: &Db, entity: &'a Self) -> Result<()> {
        use self::cases::dsl::*;

        let entity = entity.clone();
        let result = conn
            .run(move |c| {
                diesel::insert_into(cases)
                    .values(entity)
                    .returning(id)
                    .get_results::<Uuid>(c)
            })
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn update<'a>(conn: &Db, entity: &'a Self) -> Result<()> {
        use self::cases::dsl::*;

        let entity = entity.clone();
        let result = conn
            .run(move |c| diesel::update(cases).set(entity).execute(c))
            .await;

        match result {
            Ok(count) if count == 1 => Ok(()),
            Ok(_) => Err(Errors::DatabaseError(
                "Uuid is wrong, can not update".to_owned(),
            )),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn all(conn: &Db) -> Result<Vec<Case>> {
        use self::cases::dsl::*;

        conn.run(|c| cases.order(id.desc()).load::<Case>(c))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))
    }

    pub async fn get(conn: &Db, p_id: Uuid) -> Result<Option<Case>> {
        use self::cases::dsl::*;

        let result = conn
            .run(move |c| cases.find(p_id).get_result::<Case>(c))
            .await;
        match result {
            Ok(r) => Ok(Some(r)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn delete(conn: &Db, p_id: Uuid) -> Result<()> {
        use self::cases::dsl::*;

        let count = conn
            .run(move |c| diesel::delete(cases.filter(id.eq(p_id))).execute(c))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        match count {
            0 => Err(Errors::DatabaseError("id not found".to_owned())),
            _ => Ok(()),
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

impl Person {
    pub async fn new(conn: &Db, entity: NewPerson, p_case_id: Uuid) -> Result<Self> {
        use self::persons::dsl::*;

        let mut results = conn
            .run(move |c| {
                diesel::insert_into(persons)
                    .values((
                        id.eq(Uuid::from_u128(rand::random())),
                        first_name.eq(entity.first_name),
                        last_name.eq(entity.last_name),
                        father_name.eq(entity.father_name),
                        birthday.eq(entity.birthday),
                        national_number.eq(entity.national_number),
                        phone_number.eq(entity.phone_number),
                        case_id.eq(p_case_id),
                        is_leader.eq(entity.is_leader),
                        education_field.eq(entity.education_field),
                        education_location.eq(entity.education_location),
                    ))
                    .get_results(c)
            })
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        match results.pop() {
            Some(entity) => Ok(entity),
            None => Err(Errors::DatabaseError(
                "error while inserting new item to database".to_string(),
            )),
        }
    }

    pub async fn insert<'a>(conn: &Db, entity: &'a Self) -> Result<()> {
        use self::persons::dsl::*;

        let entity = entity.clone();
        let result = conn
            .run(move |c| {
                diesel::insert_into(persons)
                    .values(entity)
                    .returning(id)
                    .get_results::<Uuid>(c)
            })
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn update<'a>(conn: &Db, entity: &'a Self) -> Result<()> {
        use self::persons::dsl::*;

        let entity = entity.clone();
        let result = conn
            .run(move |c| diesel::update(persons).set(&entity).execute(c))
            .await;

        match result {
            Ok(count) if count == 1 => Ok(()),
            Ok(_) => Err(Errors::DatabaseError(
                "Uuid is wrong, can not update".to_owned(),
            )),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn all(conn: &Db) -> Result<Vec<Person>> {
        use self::persons::dsl::*;

        conn.run(|c| persons.order(id.desc()).load::<Person>(c))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))
    }

    pub async fn get(conn: &Db, p_id: Uuid) -> Result<Option<Person>> {
        use self::persons::dsl::*;

        let result = conn
            .run(move |c| persons.find(p_id).get_result::<Person>(c))
            .await;
        match result {
            Ok(r) => Ok(Some(r)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn all_by_case_id(conn: &Db, p_case_id: Uuid) -> Result<Vec<Person>> {
        use self::persons::dsl::*;

        conn.run(move |c| {
            persons
                .order(id.desc())
                .filter(case_id.eq(p_case_id))
                .load::<Person>(c)
        })
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))
    }

    pub async fn delete(conn: &Db, p_id: Uuid) -> Result<()> {
        use self::persons::dsl::*;

        let count = conn
            .run(move |c| diesel::delete(persons.filter(id.eq(p_id))).execute(c))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        match count {
            0 => Err(Errors::DatabaseError("id not found".to_owned())),
            _ => Ok(()),
        }
    }

    pub async fn set_leader(conn: &Db, person_id: Uuid) -> Result<()> {
        use self::persons::dsl::*;

        let count = conn
            .run(move |c| {
                diesel::update(persons)
                    .filter(id.eq(person_id))
                    .set(is_leader.eq(true))
                    .execute(c)
                    .map_err(|e| Errors::DatabaseError(e.to_string()))
            })
            .await?;

        match count {
            0 => Err(Errors::DatabaseError("id not found".to_owned())),
            _ => Ok(()),
        }
    }

    pub async fn clear_leader(conn: &Db, person_id: Uuid) -> Result<()> {
        use self::persons::dsl::*;

        let count = conn
            .run(move |c| {
                diesel::update(persons)
                    .filter(id.eq(person_id))
                    .set(is_leader.eq(false))
                    .execute(c)
                    .map_err(|e| Errors::DatabaseError(e.to_string()))
            })
            .await?;

        match count {
            0 => Err(Errors::DatabaseError("id not found".to_owned())),
            _ => Ok(()),
        }
    }
}

impl PersonJob {
    pub async fn new(conn: &Db, entity: NewPersonJob, p_person_id: Uuid) -> Result<Self> {
        use self::person_jobs::dsl::*;

        let mut results: Vec<PersonJob> = conn
            .run(move |c| {
                diesel::insert_into(person_jobs)
                    .values((
                        id.eq(Uuid::from_u128(rand::random())),
                        person_id.eq(p_person_id),
                        title.eq(entity.title),
                        income.eq(entity.income),
                        location.eq(entity.location),
                    ))
                    .get_results(c)
            })
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        match results.pop() {
            Some(entity) => {
                PersonJob::set_default(conn, entity.id).await?;
                Ok(entity)
            }
            None => Err(Errors::DatabaseError(
                "error while inserting new item to database".to_string(),
            )),
        }
    }

    pub async fn insert<'a>(conn: &Db, entity: &'a Self) -> Result<()> {
        use self::person_jobs::dsl::*;

        let entity = entity.clone();
        let result = conn
            .run(move |c| {
                diesel::insert_into(person_jobs)
                    .values(entity)
                    .returning(id)
                    .get_results::<Uuid>(c)
            })
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn update<'a>(conn: &Db, entity: &'a Self) -> Result<()> {
        use self::person_jobs::dsl::*;

        let entity = entity.clone();
        let result = conn
            .run(move |c| diesel::update(person_jobs).set(&entity).execute(c))
            .await;

        match result {
            Ok(count) if count == 1 => Ok(()),
            Ok(_) => Err(Errors::DatabaseError(
                "Uuid is wrong, can not update".to_owned(),
            )),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn all(conn: &Db) -> Result<Vec<PersonJob>> {
        use self::person_jobs::dsl::*;

        conn.run(|c| person_jobs.order(id.desc()).load::<PersonJob>(c))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))
    }

    pub async fn get(conn: &Db, p_id: Uuid) -> Result<Option<PersonJob>> {
        use self::person_jobs::dsl::*;

        let result = conn
            .run(move |c| person_jobs.find(p_id).get_result::<PersonJob>(c))
            .await;
        match result {
            Ok(r) => Ok(Some(r)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn all_by_person_id(conn: &Db, p_person_id: Uuid) -> Result<Vec<PersonJob>> {
        use self::person_jobs::dsl::*;

        conn.run(move |c| {
            person_jobs
                .order(id.desc())
                .filter(person_id.eq(p_person_id))
                .load::<PersonJob>(c)
        })
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))
    }

    pub async fn delete(conn: &Db, p_id: Uuid) -> Result<()> {
        use self::person_jobs::dsl::*;

        let count = conn
            .run(move |c| diesel::delete(person_jobs.filter(id.eq(p_id))).execute(c))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        match count {
            0 => Err(Errors::DatabaseError("id not found".to_owned())),
            _ => Ok(()),
        }
    }

    pub async fn set_default(conn: &Db, p_id: Uuid) -> Result<()> {
        use self::person_default_job::dsl::*;

        let _ = conn
            .run(move |c| diesel::delete(person_default_job.filter(person_id.eq(p_id))).execute(c))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        let job = PersonJob::get(conn, p_id).await?;

        match job {
            None => Err(Errors::DatabaseError("person job not found".to_owned())),
            Some(job) => {
                let _ = conn
                    .run(move |c| {
                        diesel::insert_into(person_default_job)
                            .values((person_id.eq(job.person_id), person_job_id.eq(job.id)))
                            .execute(c)
                            .map_err(|e| Errors::DatabaseError(e.to_string()))
                    })
                    .await?;
                Ok(())
            }
        }
    }
}

impl PersonSkill {
    pub async fn new(conn: &Db, entity: NewPersonSkill, p_person_id: Uuid) -> Result<Self> {
        use self::person_skills::dsl::*;

        let mut results = conn
            .run(move |c| {
                diesel::insert_into(person_skills)
                    .values((
                        id.eq(Uuid::from_u128(rand::random())),
                        person_id.eq(p_person_id),
                        skill.eq(entity.skill),
                    ))
                    .get_results(c)
            })
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        match results.pop() {
            Some(entity) => Ok(entity),
            None => Err(Errors::DatabaseError(
                "error while inserting new item to database".to_string(),
            )),
        }
    }

    pub async fn insert<'a>(conn: &Db, entity: &'a Self) -> Result<()> {
        use self::person_skills::dsl::*;

        let entity = entity.clone();
        let result = conn
            .run(move |c| {
                diesel::insert_into(person_skills)
                    .values(entity)
                    .returning(id)
                    .get_results::<Uuid>(c)
            })
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn update<'a>(conn: &Db, entity: &'a Self) -> Result<()> {
        use self::person_skills::dsl::*;

        let entity = entity.clone();
        let result = conn
            .run(move |c| diesel::update(person_skills).set(&entity).execute(c))
            .await;

        match result {
            Ok(count) if count == 1 => Ok(()),
            Ok(_) => Err(Errors::DatabaseError(
                "Uuid is wrong, can not update".to_owned(),
            )),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn all(conn: &Db) -> Result<Vec<PersonSkill>> {
        use self::person_skills::dsl::*;

        conn.run(|c| person_skills.order(id.desc()).load::<PersonSkill>(c))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))
    }

    pub async fn get(conn: &Db, p_id: Uuid) -> Result<Option<PersonSkill>> {
        use self::person_skills::dsl::*;

        let result = conn
            .run(move |c| person_skills.find(p_id).get_result::<PersonSkill>(c))
            .await;
        match result {
            Ok(r) => Ok(Some(r)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn all_by_person_id(conn: &Db, p_person_id: Uuid) -> Result<Vec<PersonSkill>> {
        use self::person_skills::dsl::*;

        conn.run(move |c| {
            person_skills
                .order(id.desc())
                .filter(person_id.eq(p_person_id))
                .load::<PersonSkill>(c)
        })
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))
    }

    pub async fn delete(conn: &Db, p_id: Uuid) -> Result<()> {
        use self::person_skills::dsl::*;

        let count = conn
            .run(move |c| diesel::delete(person_skills.filter(id.eq(p_id))).execute(c))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        match count {
            0 => Err(Errors::DatabaseError("id not found".to_owned())),
            _ => Ok(()),
        }
    }
}

impl PersonRequirement {
    pub async fn new(conn: &Db, entity: NewPersonRequirement, p_person_id: Uuid) -> Result<Self> {
        use self::person_requirements::dsl::*;

        let mut results = conn
            .run(move |c| {
                diesel::insert_into(person_requirements)
                    .values((
                        id.eq(Uuid::from_u128(rand::random())),
                        person_id.eq(p_person_id),
                        description.eq(entity.description),
                    ))
                    .get_results(c)
            })
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        match results.pop() {
            Some(entity) => Ok(entity),
            None => Err(Errors::DatabaseError(
                "error while inserting new item to database".to_string(),
            )),
        }
    }

    pub async fn insert<'a>(conn: &Db, entity: &'a Self) -> Result<()> {
        use self::person_requirements::dsl::*;

        let entity = entity.clone();
        let result = conn
            .run(move |c| {
                diesel::insert_into(person_requirements)
                    .values(entity)
                    .returning(id)
                    .get_results::<Uuid>(c)
            })
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn update<'a>(conn: &Db, entity: &'a Self) -> Result<()> {
        use self::person_requirements::dsl::*;

        let entity = entity.clone();
        let result = conn
            .run(move |c| diesel::update(person_requirements).set(&entity).execute(c))
            .await;

        match result {
            Ok(count) if count == 1 => Ok(()),
            Ok(_) => Err(Errors::DatabaseError(
                "Uuid is wrong, can not update".to_owned(),
            )),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn all(conn: &Db) -> Result<Vec<PersonRequirement>> {
        use self::person_requirements::dsl::*;

        conn.run(|c| {
            person_requirements
                .order(id.desc())
                .load::<PersonRequirement>(c)
        })
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))
    }

    pub async fn all_by_person_id(conn: &Db, p_person_id: Uuid) -> Result<Vec<PersonRequirement>> {
        use self::person_requirements::dsl::*;

        conn.run(move |c| {
            person_requirements
                .order(id.desc())
                .filter(person_id.eq(p_person_id))
                .load::<PersonRequirement>(c)
        })
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))
    }

    pub async fn get(conn: &Db, p_id: Uuid) -> Result<Option<PersonRequirement>> {
        use self::person_requirements::dsl::*;

        let result = conn
            .run(move |c| {
                person_requirements
                    .find(p_id)
                    .get_result::<PersonRequirement>(c)
            })
            .await;
        match result {
            Ok(r) => Ok(Some(r)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn delete(conn: &Db, p_id: Uuid) -> Result<()> {
        use self::person_requirements::dsl::*;

        let count = conn
            .run(move |c| diesel::delete(person_requirements.filter(id.eq(p_id))).execute(c))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        match count {
            0 => Err(Errors::DatabaseError("id not found".to_owned())),
            _ => Ok(()),
        }
    }
}
