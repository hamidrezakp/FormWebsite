use super::jwt;
use super::Db;
use crate::errors::*;
use crate::models::*;
use rocket::serde::json::Json;
use rocket::Route;
use uuid::Uuid;

#[get("/user-info")]
async fn get_info(conn: Db, token: jwt::IsLoggedIn) -> Result<Option<Json<User>>> {
    let user = User::get(&conn, token.0.user_id).await?;
    Ok(user.map(Json))
}

#[post("/", data = "<login_request>")]
async fn login(login_request: LoginRequest, conn: Db) -> Result<Json<LoginResponse>> {
    //TODO write request guards and use the user id instead.
    let case = Case::new(&conn, case.into_inner(), Uuid::nil()).await?;
    Ok(Json(case))
}

pub fn get_routes() -> Vec<Route> {
    routes![get_info, login, logout]
}
