use super::Db;
use crate::errors::*;
use crate::models::*;
use rocket::serde::json::Json;
use rocket::Route;
use uuid::Uuid;

#[get("/<id>")]
async fn get(id: Uuid, conn: Db) -> Result<Option<Json<Case>>> {
    let case = Case::get(&conn, id).await?;
    Ok(case.map(|i| Json(i)))
}

#[get("/")]
async fn get_all(conn: Db) -> Result<Json<Vec<Case>>> {
    //TODO: Needs pagination
    let cases = Case::all(&conn).await?;
    Ok(Json(cases))
}

#[post("/", data = "<case>")]
async fn insert(case: Json<NewCase>, conn: Db) -> Result<Json<Case>> {
    //TODO write request guards and use the user id instead.
    let case = Case::new(&conn, case.into_inner(), Uuid::nil()).await?;
    Ok(Json(case))
}

#[put("/", data = "<case>")]
async fn update(case: Json<Case>, conn: Db) -> Result<()> {
    case.into_inner().update(&conn).await
}

#[delete("/<id>")]
async fn delete(id: Uuid, conn: Db) -> Result<()> {
    Case::delete(&conn, id).await
}

#[patch("/<id>/activate")]
async fn activate(id: Uuid, conn: Db) -> Result<()> {
    let case = Case::get(&conn, id).await?;
    match case {
        None => Err(Errors::BadRequest("invalid id".to_owned())),
        Some(case) => case.activate().update(&conn).await,
    }
}

#[patch("/<id>/deactivate")]
async fn deactivate(id: Uuid, conn: Db) -> Result<()> {
    let case = Case::get(&conn, id).await?;
    match case {
        None => Err(Errors::BadRequest("invalid id".to_owned())),
        Some(case) => case.deactivate().update(&conn).await,
    }
}

#[get("/<id>/person")]
async fn get_all_persons(id: Uuid, conn: Db) -> Result<Json<Vec<Person>>> {
    let persons = Person::all_by_case_id(&conn, id).await;
    persons.map(|i| Json(i))
}

#[get("/<id>/action")]
async fn get_all_actions(id: Uuid, conn: Db) -> Result<Json<Vec<CaseAction>>> {
    let actions = CaseAction::all_by_case_id(&conn, id).await;
    actions.map(|i| Json(i))
}

#[get("/<id>/action/week")]
async fn get_week_actions(id: Uuid, conn: Db) -> Result<Json<Vec<CaseAction>>> {
    let actions = CaseAction::week_actions_for_case(&conn, id).await;
    actions.map(|i| Json(i))
}

#[get("/<id>/action/today")]
async fn get_today_actions(id: Uuid, conn: Db) -> Result<Json<Vec<CaseAction>>> {
    let actions = CaseAction::today_actions_for_case(&conn, id).await;
    actions.map(|i| Json(i))
}

pub fn get_routes() -> Vec<Route> {
    routes![
        get,
        get_all,
        get_all_persons,
        insert,
        update,
        delete,
        activate,
        deactivate,
        get_all_actions,
        get_week_actions,
        get_today_actions
    ]
}
