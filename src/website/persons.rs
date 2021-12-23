use super::Db;
use crate::errors::*;
use crate::models::*;
use rocket::serde::json::Json;
use rocket::Route;
use uuid::Uuid;

#[get("/<id>")]
async fn get(id: Uuid, conn: Db) -> Result<Option<Json<Person>>> {
    let person = Person::get(&conn, id).await?;
    Ok(person.map(|i| Json(i)))
}

#[get("/")]
async fn get_all(conn: Db) -> Result<Json<Vec<Person>>> {
    //TODO: Needs pagination
    let persons = Person::all(&conn).await;
    persons.map(|i| Json(i))
}

#[post("/?<case_id>", data = "<person>")]
async fn insert(case_id: Uuid, person: Json<NewPerson>, conn: Db) -> Result<Json<Person>> {
    let person = Person::new(&conn, person.into_inner(), case_id).await?;
    Ok(Json(person))
}

#[put("/", data = "<person>")]
async fn update(person: Json<Person>, conn: Db) -> Result<()> {
    Person::update(&conn, &person.into_inner()).await
}

#[delete("/<id>")]
async fn delete(id: Uuid, conn: Db) -> Result<()> {
    Person::delete(&conn, id).await
}

#[get("/<_id>/skill/<skill_id>")]
async fn get_skill(_id: Uuid, skill_id: Uuid, conn: Db) -> Result<Option<Json<PersonSkill>>> {
    let skill = PersonSkill::get(&conn, skill_id).await?;
    Ok(skill.map(|i| Json(i)))
}

#[get("/<id>/skill")]
async fn get_all_skills(id: Uuid, conn: Db) -> Result<Json<Vec<PersonSkill>>> {
    let skills = PersonSkill::all_by_person_id(&conn, id).await?;
    Ok(Json(skills))
}

#[post("/<id>/skill", data = "<skill>")]
async fn insert_skill(
    id: Uuid,
    skill: Json<NewPersonSkill>,
    conn: Db,
) -> Result<Json<PersonSkill>> {
    let skill = PersonSkill::new(&conn, skill.into_inner(), id).await?;
    Ok(Json(skill))
}

#[put("/<_id>/skill", data = "<skill>")]
async fn update_skill(_id: Uuid, skill: Json<PersonSkill>, conn: Db) -> Result<()> {
    PersonSkill::update(&conn, &skill.into_inner()).await
}

#[delete("/<_id>/skill/<skill_id>")]
async fn delete_skill(_id: Uuid, skill_id: Uuid, conn: Db) -> Result<()> {
    PersonSkill::delete(&conn, skill_id).await
}

#[get("/<_id>/job/<job_id>")]
async fn get_job(_id: Uuid, job_id: Uuid, conn: Db) -> Result<Option<Json<PersonJob>>> {
    let job = PersonJob::get(&conn, job_id).await?;
    Ok(job.map(|i| Json(i)))
}

#[get("/<id>/job")]
async fn get_all_jobs(id: Uuid, conn: Db) -> Result<Json<Vec<PersonJob>>> {
    let jobs = PersonJob::all_by_person_id(&conn, id).await?;
    Ok(Json(jobs))
}

#[post("/<id>/job", data = "<job>")]
async fn insert_job(id: Uuid, job: Json<NewPersonJob>, conn: Db) -> Result<Json<PersonJob>> {
    let job = PersonJob::new(&conn, job.into_inner(), id).await?;
    Ok(Json(job))
}

#[put("/<_id>/job", data = "<job>")]
async fn update_job(_id: Uuid, job: Json<PersonJob>, conn: Db) -> Result<()> {
    PersonJob::update(&conn, &job.into_inner()).await
}

#[delete("/<_id>/job/<job_id>")]
async fn delete_job(_id: Uuid, job_id: Uuid, conn: Db) -> Result<()> {
    PersonJob::delete(&conn, job_id).await
}

#[get("/<_id>/requirement/<requirement_id>")]
async fn get_requirement(
    _id: Uuid,
    requirement_id: Uuid,
    conn: Db,
) -> Result<Option<Json<PersonRequirement>>> {
    let requirement = PersonRequirement::get(&conn, requirement_id).await?;
    Ok(requirement.map(|i| Json(i)))
}

#[get("/<id>/requirement")]
async fn get_all_requirements(id: Uuid, conn: Db) -> Result<Json<Vec<PersonRequirement>>> {
    let requirements = PersonRequirement::all_by_person_id(&conn, id).await?;
    Ok(Json(requirements))
}

#[post("/<id>/requirement", data = "<requirement>")]
async fn insert_requirement(
    id: Uuid,
    requirement: Json<NewPersonRequirement>,
    conn: Db,
) -> Result<Json<PersonRequirement>> {
    let requirement = PersonRequirement::new(&conn, requirement.into_inner(), id).await?;
    Ok(Json(requirement))
}

#[put("/<_id>/requirement", data = "<requirement>")]
async fn update_requirement(
    _id: Uuid,
    requirement: Json<PersonRequirement>,
    conn: Db,
) -> Result<()> {
    PersonRequirement::update(&conn, &requirement.into_inner()).await
}

#[delete("/<_id>/requirement/<requirement_id>")]
async fn delete_requirement(_id: Uuid, requirement_id: Uuid, conn: Db) -> Result<()> {
    PersonRequirement::delete(&conn, requirement_id).await
}

pub fn get_routes() -> Vec<Route> {
    routes![
        get,
        get_all,
        insert,
        update,
        delete,
        get_skill,
        get_all_skills,
        insert_skill,
        update_skill,
        delete_skill,
        get_job,
        get_all_jobs,
        insert_job,
        update_job,
        delete_job,
        get_requirement,
        get_all_requirements,
        insert_requirement,
        update_requirement,
        delete_requirement,
    ]
}
