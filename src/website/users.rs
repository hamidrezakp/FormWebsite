use self::models::UserInfo;

use super::jwt;
use super::Db;
use crate::errors::*;
use crate::models::*;
use rocket::serde::json::Json;
use rocket::Route;
use uuid::Uuid;

mod models {
    use crate::{models::User, website::jwt::Role};
    use serde::Serialize;
    use uuid::Uuid;

    #[derive(Serialize)]
    pub struct UserInfo {
        pub id: Uuid,
        pub username: String,
        pub first_name: String,
        pub last_name: String,
        pub role: Role,
    }

    impl UserInfo {
        pub fn of_user(user: User) -> Self {
            Self {
                id: user.id,
                username: user.username,
                first_name: user.first_name,
                last_name: user.last_name,
                role: user.role.into(),
            }
        }
    }
}

#[get("/<id>")]
async fn get(id: Uuid, conn: Db, _admin: jwt::IsAdmin) -> Result<Option<Json<UserInfo>>> {
    let user = User::get(&conn, id).await?;
    Ok(user.map(UserInfo::of_user).map(Json))
}

#[get("/")]
async fn get_all(conn: Db, _admin: jwt::IsAdmin) -> Result<Json<Vec<UserInfo>>> {
    //TODO: Needs pagination
    let mut results = Vec::new();

    for user in User::all(&conn).await? {
        results.push(UserInfo::of_user(user));
    }

    Ok(Json(results))
}

#[post("/", data = "<user>")]
async fn insert(user: Json<NewUser>, conn: Db, _admin: jwt::IsAdmin) -> Result<Json<UserInfo>> {
    let user = User::new(&conn, user.into_inner()).await?;
    Ok(Json(UserInfo::of_user(user)))
}

pub fn get_routes() -> Vec<Route> {
    routes![get, get_all, insert]
}
