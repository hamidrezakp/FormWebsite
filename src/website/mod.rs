use rocket_sync_db_pools::database;

mod cases;
mod person_jobs;
mod person_requirements;
mod person_skills;
mod persons;
mod users;

#[database("form_website")]
pub struct Db(diesel::PgConnection);

pub async fn run() -> std::result::Result<(), rocket::Error> {
    rocket::build()
        .mount("/case", cases::get_routes())
        .mount("/person", persons::get_routes())
        .mount("/person-job", person_jobs::get_routes())
        .mount("/person-skill", person_skills::get_routes())
        .mount("/person-requirement", person_requirements::get_routes())
        .attach(Db::fairing())
        .ignite()
        .await?
        .launch()
        .await
}
