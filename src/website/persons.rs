use super::jwt::HasEditorPermissions;
use super::Db;
use crate::errors::*;
use crate::models::*;
use rocket::serde::json::Json;
use rocket::Route;
use uuid::Uuid;

#[get("/<id>")]
async fn get(id: Uuid, conn: Db, _token: HasEditorPermissions) -> Result<Option<Json<Person>>> {
    let person = Person::get(&conn, id).await?;
    Ok(person.map(Json))
}

#[get("/")]
async fn get_all(conn: Db, _token: HasEditorPermissions) -> Result<Json<Vec<Person>>> {
    //TODO: Needs pagination
    let persons = Person::all(&conn).await;
    persons.map(Json)
}

#[post("/", data = "<person>")]
async fn insert(
    person: Json<NewPerson>,
    conn: Db,
    _token: HasEditorPermissions,
) -> Result<Json<Person>> {
    let person = Person::new(&conn, person.into_inner()).await?;
    Ok(Json(person))
}

#[put("/", data = "<person>")]
async fn update(person: Json<Person>, conn: Db, _token: HasEditorPermissions) -> Result<()> {
    person.into_inner().update(&conn).await
}

#[delete("/<id>")]
async fn delete(id: Uuid, conn: Db, _token: HasEditorPermissions) -> Result<()> {
    Person::delete(&conn, id).await
}

#[post("/<id>/set-leader")]
async fn set_leader(id: Uuid, conn: Db, _token: HasEditorPermissions) -> Result<()> {
    Person::set_leader(&conn, id).await
}

#[post("/<id>/clear-leader")]
async fn clear_leader(id: Uuid, conn: Db, _token: HasEditorPermissions) -> Result<()> {
    Person::clear_leader(&conn, id).await
}

#[get("/<id>/job")]
async fn get_jobs(
    id: Uuid,
    conn: Db,
    _token: HasEditorPermissions,
) -> Result<Json<Vec<PersonJob>>> {
    let jobs = PersonJob::all_by_person_id(&conn, id).await?;
    Ok(Json(jobs))
}

#[get("/<id>/requirement")]
async fn get_requirements(
    id: Uuid,
    conn: Db,
    _token: HasEditorPermissions,
) -> Result<Json<Vec<PersonRequirement>>> {
    let requirements = PersonRequirement::all_by_person_id(&conn, id).await?;
    Ok(Json(requirements))
}

#[get("/<id>/skill")]
async fn get_skills(
    id: Uuid,
    conn: Db,
    _token: HasEditorPermissions,
) -> Result<Json<Vec<PersonSkill>>> {
    let skills = PersonSkill::all_by_person_id(&conn, id).await?;
    Ok(Json(skills))
}

pub fn get_routes() -> Vec<Route> {
    routes![
        get,
        get_all,
        insert,
        update,
        delete,
        set_leader,
        clear_leader,
        get_requirements,
        get_jobs,
        get_skills,
    ]
}
