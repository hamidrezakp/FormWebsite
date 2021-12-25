use super::Db;
use crate::errors::*;
use crate::models::*;
use rocket::serde::json::Json;
use rocket::Route;
use uuid::Uuid;

#[get("/<id>")]
async fn get(id: Uuid, conn: Db) -> Result<Option<Json<PersonRequirement>>> {
    let requirement = PersonRequirement::get(&conn, id).await?;
    Ok(requirement.map(|i| Json(i)))
}

#[post("/?<person_id>", data = "<requirement>")]
async fn insert(
    person_id: Uuid,
    requirement: Json<NewPersonRequirement>,
    conn: Db,
) -> Result<Json<PersonRequirement>> {
    let requirement = PersonRequirement::new(&conn, requirement.into_inner(), person_id).await?;
    Ok(Json(requirement))
}

#[put("/", data = "<requirement>")]
async fn update(requirement: Json<PersonRequirement>, conn: Db) -> Result<()> {
    requirement.into_inner().update(&conn).await
}

#[delete("/<id>")]
async fn delete(id: Uuid, conn: Db) -> Result<()> {
    PersonRequirement::delete(&conn, id).await
}

pub fn get_routes() -> Vec<Route> {
    routes![get, insert, update, delete]
}
