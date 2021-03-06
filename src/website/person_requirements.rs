use super::jwt::HasEditorPermissions;
use super::Db;
use crate::errors::*;
use crate::models::*;
use rocket::serde::json::Json;
use rocket::Route;
use uuid::Uuid;

#[get("/<id>")]
async fn get(
    id: Uuid,
    conn: Db,
    _token: HasEditorPermissions,
) -> Result<Option<Json<PersonRequirement>>> {
    let requirement = PersonRequirement::get(&conn, id).await?;
    Ok(requirement.map(Json))
}

#[post("/", data = "<requirement>")]
async fn insert(
    requirement: Json<NewPersonRequirement>,
    conn: Db,
    _token: HasEditorPermissions,
) -> Result<Json<PersonRequirement>> {
    let requirement = PersonRequirement::new(&conn, requirement.into_inner()).await?;
    Ok(Json(requirement))
}

#[put("/", data = "<requirement>")]
async fn update(
    requirement: Json<PersonRequirement>,
    conn: Db,
    _token: HasEditorPermissions,
) -> Result<()> {
    requirement.into_inner().update(&conn).await
}

#[delete("/<id>")]
async fn delete(id: Uuid, conn: Db, _token: HasEditorPermissions) -> Result<()> {
    PersonRequirement::delete(&conn, id).await
}

pub fn get_routes() -> Vec<Route> {
    routes![get, insert, update, delete]
}
