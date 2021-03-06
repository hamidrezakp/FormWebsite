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
) -> Result<Option<Json<PersonSkill>>> {
    let skill = PersonSkill::get(&conn, id).await?;
    Ok(skill.map(Json))
}

#[post("/", data = "<skill>")]
async fn insert(
    skill: Json<NewPersonSkill>,
    conn: Db,
    _token: HasEditorPermissions,
) -> Result<Json<PersonSkill>> {
    let skill = PersonSkill::new(&conn, skill.into_inner()).await?;
    Ok(Json(skill))
}

#[put("/", data = "<skill>")]
async fn update(skill: Json<PersonSkill>, conn: Db, _token: HasEditorPermissions) -> Result<()> {
    skill.into_inner().update(&conn).await
}

#[delete("/<id>")]
async fn delete(id: Uuid, conn: Db, _token: HasEditorPermissions) -> Result<()> {
    PersonSkill::delete(&conn, id).await
}

pub fn get_routes() -> Vec<Route> {
    routes![get, insert, update, delete]
}
