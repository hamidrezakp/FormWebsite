use super::jwt::HasEditorPermissions;
use super::Db;
use crate::errors::*;
use crate::models::*;
use rocket::serde::json::Json;
use rocket::Route;
use uuid::Uuid;

#[get("/")]
async fn get_all(conn: Db, _token: HasEditorPermissions) -> Result<Json<Vec<CaseAction>>> {
    let actions = CaseAction::all(&conn).await?;
    Ok(Json(actions))
}

#[get("/<id>")]
async fn get(id: Uuid, conn: Db, _token: HasEditorPermissions) -> Result<Option<Json<CaseAction>>> {
    let case_action = CaseAction::get(&conn, id).await?;
    Ok(case_action.map(Json))
}

#[post("/", data = "<case_action>")]
async fn insert(
    case_action: Json<NewCaseAction>,
    conn: Db,
    _token: HasEditorPermissions,
) -> Result<Json<CaseAction>> {
    let case_action = CaseAction::new(&conn, case_action.into_inner()).await?;
    Ok(Json(case_action))
}

#[put("/", data = "<case_action>")]
async fn update(
    case_action: Json<CaseAction>,
    conn: Db,
    _token: HasEditorPermissions,
) -> Result<()> {
    case_action.into_inner().update(&conn).await
}

#[delete("/<id>")]
async fn delete(id: Uuid, conn: Db, _token: HasEditorPermissions) -> Result<()> {
    CaseAction::delete(&conn, id).await
}

pub fn get_routes() -> Vec<Route> {
    routes![get, get_all, insert, update, delete]
}
