use super::errors::*;
use super::schema::*;
use super::website::Db;
use chrono::prelude::*;
use chrono::Duration;
use diesel::prelude::*;
use diesel::Insertable;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

const ACTION_STATUS_DONE: i32 = 2;

type Toman = i32;

#[derive(
    Debug, Queryable, Serialize, Deserialize, Insertable, Clone, Identifiable, AsChangeset,
)]
#[changeset_options(treat_none_as_null = "true")]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: Vec<u8>,
    pub password_salt: Vec<u8>,
    pub role: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewUser {
    username: String,
    first_name: String,
    last_name: String,
    password: String,
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
    description: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewCase {
    address: Option<String>,
    description: Option<String>,
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
    birthday: NaiveDate,
    national_number: String,
    phone_number: String,
    case_id: Uuid,
    is_leader: bool,
    family_role: i32,
    description: Option<String>,
    education_field: Option<String>,
    education_location: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewPerson {
    first_name: String,
    last_name: String,
    father_name: String,
    birthday: NaiveDate,
    national_number: String,
    phone_number: String,
    case_id: Uuid,
    is_leader: bool,
    family_role: i32,
    description: Option<String>,
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

#[derive(Debug, Queryable, Serialize, Deserialize, Insertable, AsChangeset, Clone)]
#[changeset_options(treat_none_as_null = "true")]
pub struct CaseAction {
    id: Uuid,
    case_id: Uuid,
    action: String,
    status: i32,
    action_date: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewCaseAction {
    case_id: Uuid,
    action: String,
    action_date: Option<NaiveDateTime>,
}

impl User {
    pub async fn new(conn: &Db, entity: NewUser) -> Result<Self> {
        use self::users::dsl::*;
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        let salt: [u8; 32] = rand::random();
        hasher.update(entity.password.as_bytes());
        hasher.update(salt);
        let pass_hash = hasher.finalize();

        let mut results = conn
            .run(move |c| {
                diesel::insert_into(users)
                    .values((
                        id.eq(Uuid::from_u128(rand::random())),
                        username.eq(entity.username),
                        first_name.eq(entity.first_name),
                        last_name.eq(entity.last_name),
                        password_hash.eq(pass_hash.as_slice()),
                        password_salt.eq(salt.as_slice()),
                        role.eq(2),
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

    pub async fn insert(self, conn: &Db) -> Result<()> {
        use self::users::dsl::*;

        let result = conn
            .run(move |c| {
                diesel::insert_into(users)
                    .values(self)
                    .returning(id)
                    .get_results::<Uuid>(c)
            })
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn update(self, conn: &Db) -> Result<()> {
        use self::users::dsl::*;

        let result = conn
            .run(move |c| {
                diesel::update(users)
                    .filter(id.eq(self.id))
                    .set(self)
                    .execute(c)
            })
            .await;

        match result {
            Ok(count) if count == 1 => Ok(()),
            Ok(_) => Err(Errors::BadRequest("id not found".to_owned())),
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

    pub async fn get_by_username(conn: &Db, p_username: String) -> Result<Option<User>> {
        use self::users::dsl::*;

        let result = conn
            .run(move |c| users.filter(username.eq(p_username)).get_result::<User>(c))
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

    pub async fn insert(self, conn: &Db) -> Result<()> {
        use self::cases::dsl::*;

        let result = conn
            .run(move |c| {
                diesel::insert_into(cases)
                    .values(self)
                    .returning(id)
                    .get_results::<Uuid>(c)
            })
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn update(self, conn: &Db) -> Result<()> {
        use self::cases::dsl::*;

        let result = conn
            .run(move |c| {
                diesel::update(cases)
                    .filter(id.eq(self.id))
                    .set(self)
                    .execute(c)
            })
            .await;

        match result {
            Ok(count) if count == 1 => Ok(()),
            Ok(_) => Err(Errors::BadRequest("id not found".to_owned())),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn all(conn: &Db) -> Result<Vec<Case>> {
        use self::cases::dsl::*;

        conn.run(|c| cases.order(registration_date.desc()).load::<Case>(c))
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
        let persons_count = {
            use self::persons::dsl::*;

            let result = conn
                .run(move |c| {
                    persons
                        .count()
                        .filter(case_id.eq(p_id))
                        .get_result::<i64>(c)
                })
                .await;

            match result {
                Ok(c) => Ok(c),
                Err(e) => Err(Errors::DatabaseError(e.to_string())),
            }
        }?;

        if persons_count != 0 {
            return Err(Errors::BadRequest(
                "remove persons within case before removing case".to_owned(),
            ));
        }

        let count = {
            use self::cases::dsl::*;
            conn.run(move |c| diesel::delete(cases.filter(id.eq(p_id))).execute(c))
                .await
                .map_err(|e| Errors::DatabaseError(e.to_string()))?
        };

        match count {
            0 => Err(Errors::BadRequest("id not found".to_owned())),
            _ => Ok(()),
        }
    }

    pub fn activate(self) -> Self {
        Self {
            active: true,
            ..self
        }
    }

    pub fn deactivate(self) -> Self {
        Self {
            active: false,
            ..self
        }
    }
}

impl Person {
    pub async fn new(conn: &Db, entity: NewPerson) -> Result<Self> {
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
                        case_id.eq(entity.case_id),
                        is_leader.eq(entity.is_leader),
                        description.eq(entity.description),
                        family_role.eq(entity.family_role),
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

    pub async fn insert(self, conn: &Db) -> Result<()> {
        use self::persons::dsl::*;

        let result = conn
            .run(move |c| {
                diesel::insert_into(persons)
                    .values(self)
                    .returning(id)
                    .get_results::<Uuid>(c)
            })
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn update(self, conn: &Db) -> Result<()> {
        use self::persons::dsl::*;

        let result = conn
            .run(move |c| {
                diesel::update(persons)
                    .filter(id.eq(self.id))
                    .set(self)
                    .execute(c)
            })
            .await;

        match result {
            Ok(count) if count == 1 => Ok(()),
            Ok(_) => Err(Errors::BadRequest("id not found".to_owned())),
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
            0 => Err(Errors::BadRequest("id not found".to_owned())),
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
            0 => Err(Errors::BadRequest("id not found".to_owned())),
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
            0 => Err(Errors::BadRequest("id not found".to_owned())),
            _ => Ok(()),
        }
    }
}

impl PersonJob {
    pub async fn new(conn: &Db, entity: NewPersonJob) -> Result<Self> {
        use self::person_jobs::dsl::*;

        let mut results: Vec<PersonJob> = conn
            .run(move |c| {
                diesel::insert_into(person_jobs)
                    .values((
                        id.eq(Uuid::from_u128(rand::random())),
                        person_id.eq(entity.person_id),
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

    pub async fn insert(self, conn: &Db) -> Result<()> {
        use self::person_jobs::dsl::*;

        let result = conn
            .run(move |c| {
                diesel::insert_into(person_jobs)
                    .values(self)
                    .returning(id)
                    .get_results::<Uuid>(c)
            })
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn update(self, conn: &Db) -> Result<()> {
        use self::person_jobs::dsl::*;

        let result = conn
            .run(move |c| {
                diesel::update(person_jobs)
                    .filter(id.eq(self.id))
                    .set(self)
                    .execute(c)
            })
            .await;

        match result {
            Ok(count) if count == 1 => Ok(()),
            Ok(_) => Err(Errors::BadRequest("id not found".to_owned())),
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
            0 => Err(Errors::BadRequest("id not found".to_owned())),
            _ => Ok(()),
        }
    }

    pub async fn set_default(conn: &Db, p_id: Uuid) -> Result<()> {
        use self::person_default_job::dsl::*;

        let _ = conn
            .run(move |c| {
                diesel::delete(person_default_job.filter(person_job_id.eq(p_id))).execute(c)
            })
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        let job = PersonJob::get(conn, p_id).await?;

        match job {
            None => Err(Errors::BadRequest("id not found".to_owned())),
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
    pub async fn new(conn: &Db, entity: NewPersonSkill) -> Result<Self> {
        use self::person_skills::dsl::*;

        let mut results = conn
            .run(move |c| {
                diesel::insert_into(person_skills)
                    .values((
                        id.eq(Uuid::from_u128(rand::random())),
                        person_id.eq(entity.person_id),
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

    pub async fn insert(self, conn: &Db) -> Result<()> {
        use self::person_skills::dsl::*;

        let result = conn
            .run(move |c| {
                diesel::insert_into(person_skills)
                    .values(self)
                    .returning(id)
                    .get_results::<Uuid>(c)
            })
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn update(self, conn: &Db) -> Result<()> {
        use self::person_skills::dsl::*;

        let result = conn
            .run(move |c| {
                diesel::update(person_skills)
                    .filter(id.eq(self.id))
                    .set(self)
                    .execute(c)
            })
            .await;

        match result {
            Ok(count) if count == 1 => Ok(()),
            Ok(_) => Err(Errors::BadRequest("id not found".to_owned())),
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
            0 => Err(Errors::BadRequest("id not found".to_owned())),
            _ => Ok(()),
        }
    }
}

impl PersonRequirement {
    pub async fn new(conn: &Db, entity: NewPersonRequirement) -> Result<Self> {
        use self::person_requirements::dsl::*;

        let mut results = conn
            .run(move |c| {
                diesel::insert_into(person_requirements)
                    .values((
                        id.eq(Uuid::from_u128(rand::random())),
                        person_id.eq(entity.person_id),
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

    pub async fn insert(self, conn: &Db) -> Result<()> {
        use self::person_requirements::dsl::*;

        let result = conn
            .run(move |c| {
                diesel::insert_into(person_requirements)
                    .values(self)
                    .returning(id)
                    .get_results::<Uuid>(c)
            })
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn update(self, conn: &Db) -> Result<()> {
        use self::person_requirements::dsl::*;

        let result = conn
            .run(move |c| {
                diesel::update(person_requirements)
                    .filter(id.eq(self.id))
                    .set(self)
                    .execute(c)
            })
            .await;

        match result {
            Ok(count) if count == 1 => Ok(()),
            Ok(_) => Err(Errors::BadRequest("id not found".to_owned())),
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
            0 => Err(Errors::BadRequest("id not found".to_owned())),
            _ => Ok(()),
        }
    }
}

impl CaseAction {
    pub async fn new(conn: &Db, entity: NewCaseAction) -> Result<Self> {
        use self::case_actions::dsl::*;

        let mut results = conn
            .run(move |c| {
                diesel::insert_into(case_actions)
                    .values((
                        id.eq(Uuid::from_u128(rand::random())),
                        case_id.eq(entity.case_id),
                        action.eq(entity.action),
                        action_date.eq(entity.action_date),
                        status.eq(0),
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

    pub async fn insert(self, conn: &Db) -> Result<()> {
        use self::case_actions::dsl::*;

        let result = conn
            .run(move |c| {
                diesel::insert_into(case_actions)
                    .values(self)
                    .returning(id)
                    .get_results::<Uuid>(c)
            })
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn update(self, conn: &Db) -> Result<()> {
        use self::case_actions::dsl::*;

        let result = conn
            .run(move |c| {
                diesel::update(case_actions)
                    .filter(id.eq(self.id))
                    .set(self)
                    .execute(c)
            })
            .await;

        match result {
            Ok(count) if count == 1 => Ok(()),
            Ok(_) => Err(Errors::BadRequest("id not found".to_owned())),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn all(conn: &Db) -> Result<Vec<CaseAction>> {
        use self::case_actions::dsl::*;

        let mut result = conn
            .run(|c| case_actions.order(action_date.desc()).load::<CaseAction>(c))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        result.sort_by(|i, j| i.action_date.cmp(&j.action_date));
        Ok(result)
    }

    pub async fn all_by_case_id(conn: &Db, p_case_id: Uuid) -> Result<Vec<CaseAction>> {
        use self::case_actions::dsl::*;

        let mut result = conn
            .run(move |c| {
                case_actions
                    .order(action_date.desc())
                    .filter(case_id.eq(p_case_id))
                    .load::<CaseAction>(c)
            })
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        result.sort_by(|i, j| i.action_date.cmp(&j.action_date));
        Ok(result)
    }

    pub async fn get(conn: &Db, p_id: Uuid) -> Result<Option<CaseAction>> {
        use self::case_actions::dsl::*;

        let result = conn
            .run(move |c| case_actions.find(p_id).get_result::<CaseAction>(c))
            .await;
        match result {
            Ok(r) => Ok(Some(r)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(Errors::DatabaseError(e.to_string())),
        }
    }

    pub async fn delete(conn: &Db, p_id: Uuid) -> Result<()> {
        use self::case_actions::dsl::*;

        let count = conn
            .run(move |c| diesel::delete(case_actions.filter(id.eq(p_id))).execute(c))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        match count {
            0 => Err(Errors::BadRequest("id not found".to_owned())),
            _ => Ok(()),
        }
    }

    pub async fn today_actions_for_case(conn: &Db, p_case_id: Uuid) -> Result<Vec<CaseAction>> {
        use self::case_actions::dsl::*;

        let today = Utc::now().date().naive_utc().and_hms(0, 0, 0);
        let tomarrow = today + Duration::days(1);

        let mut result = conn
            .run(move |c| {
                case_actions
                    .order(action_date)
                    .filter(case_id.eq(p_case_id))
                    .filter(action_date.gt(today))
                    .filter(action_date.lt(tomarrow))
                    .filter(status.lt(ACTION_STATUS_DONE))
                    .load::<CaseAction>(c)
            })
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        result.sort_by(|i, j| i.action_date.cmp(&j.action_date));
        Ok(result)
    }

    pub async fn today_actions_for_all_cases(conn: &Db) -> Result<Vec<CaseAction>> {
        use self::case_actions::dsl::*;

        let today = Utc::now().date().naive_utc().and_hms(0, 0, 0);
        let tomarrow = today + Duration::days(1);

        let mut result = conn
            .run(move |c| {
                case_actions
                    .order(action_date)
                    .filter(action_date.gt(today))
                    .filter(action_date.lt(tomarrow))
                    .filter(status.lt(ACTION_STATUS_DONE))
                    .load::<CaseAction>(c)
            })
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        result.sort_by(|i, j| i.action_date.cmp(&j.action_date));
        Ok(result)
    }

    pub async fn week_actions_for_all_cases(conn: &Db) -> Result<Vec<CaseAction>> {
        use self::case_actions::dsl::*;

        let today = Utc::now().date().naive_utc().and_hms(0, 0, 0);
        let next_week = today + Duration::days(7);

        let mut result = conn
            .run(move |c| {
                case_actions
                    .order(action_date)
                    .filter(action_date.gt(today))
                    .filter(action_date.lt(next_week))
                    .filter(status.lt(ACTION_STATUS_DONE))
                    .load::<CaseAction>(c)
            })
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        result.sort_by(|i, j| i.action_date.cmp(&j.action_date));
        Ok(result)
    }

    pub async fn week_actions_for_case(conn: &Db, p_case_id: Uuid) -> Result<Vec<CaseAction>> {
        use self::case_actions::dsl::*;

        let today = Utc::now().date().naive_utc().and_hms(0, 0, 0);
        let next_week = today + Duration::days(7);

        let mut result = conn
            .run(move |c| {
                case_actions
                    .order(action_date)
                    .filter(case_id.eq(p_case_id))
                    .filter(action_date.gt(today))
                    .filter(action_date.lt(next_week))
                    .filter(status.lt(ACTION_STATUS_DONE))
                    .load::<CaseAction>(c)
            })
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;

        result.sort_by(|i, j| i.action_date.cmp(&j.action_date));
        Ok(result)
    }
}
