use super::jwt::HasEditorPermissions;
use super::Db;
use crate::errors::*;
use crate::models::*;
use rocket::serde::json::Json;
use rocket::Route;
use uuid::Uuid;

#[get("/<id>")]
async fn get(id: Uuid, conn: Db, _token: HasEditorPermissions) -> Result<Option<Json<PersonJob>>> {
    let job = PersonJob::get(&conn, id).await?;
    Ok(job.map(Json))
}

#[post("/", data = "<job>")]
async fn insert(
    job: Json<NewPersonJob>,
    conn: Db,
    _token: HasEditorPermissions,
) -> Result<Json<PersonJob>> {
    let job = PersonJob::new(&conn, job.into_inner()).await?;
    Ok(Json(job))
}

#[put("/", data = "<job>")]
async fn update(job: Json<PersonJob>, conn: Db, _token: HasEditorPermissions) -> Result<()> {
    job.into_inner().update(&conn).await
}

#[delete("/<id>")]
async fn delete(id: Uuid, conn: Db) -> Result<()> {
    PersonJob::delete(&conn, id).await
}

#[post("/<id>/set-default")]
async fn set_default(id: Uuid, conn: Db, _token: HasEditorPermissions) -> Result<()> {
    PersonJob::set_default(&conn, id).await
}

pub fn get_routes() -> Vec<Route> {
    routes![get, insert, update, delete, set_default]
}
