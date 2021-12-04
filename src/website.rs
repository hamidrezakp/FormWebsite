use super::errors::*;
use super::models::*;
use rocket::serde::json::Json;
use rocket_sync_db_pools::database;
use uuid::Uuid;

#[database("form_website")]
pub struct Db(diesel::PgConnection);

#[get("/<id>")]
async fn get_case(id: Uuid, conn: Db) -> Result<Option<Json<Case>>> {
    let case = Case::get(&conn, id).await;
    case.map(|i| i.map(|j| Json(j)))
}

#[get("/")]
async fn get_all_cases(conn: Db) -> Result<Json<Vec<Case>>> {
    let cases = Case::all(&conn).await;
    cases.map(|i| Json(i))
}

#[post("/", data = "<case>")]
async fn insert_case(case: Json<NewCase>, conn: Db) -> Result<()> {
    //TODO write request guards and use the user id instead.
    let case = Case::new(case.into_inner(), Uuid::nil());
    Case::insert(&conn, case).await
}

#[get("/<id>")]
async fn get_person(id: Uuid, conn: Db) -> Result<Option<Json<Person>>> {
    let person = Person::get(&conn, id).await;
    person.map(|i| i.map(|j| Json(j)))
}

#[post("/", data = "<person>")]
async fn insert_person(person: Json<NewPerson>, conn: Db) -> Result<()> {
    let person = Person::new(person.into_inner());
    Person::insert(&conn, person).await
}

pub async fn run() -> std::result::Result<(), rocket::Error> {
    rocket::build()
        .mount("/case", routes![get_case, get_all_cases, insert_case])
        .mount("/person", routes![get_person, insert_person])
        .attach(Db::fairing())
        .ignite()
        .await?
        .launch()
        .await
}
