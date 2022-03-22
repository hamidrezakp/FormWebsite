use rocket_sync_db_pools::database;

mod auth;
mod case_actions;
mod cases;
mod cors;
mod jwt;
mod person_jobs;
mod person_requirements;
mod person_skills;
mod persons;
mod users;

#[database("form_website")]
pub struct Db(diesel::PgConnection);

pub async fn run() -> std::result::Result<(), rocket::Error> {
    let rocket = rocket::build()
        .mount("/auth", auth::get_routes())
        .mount("/user", users::get_routes())
        .mount("/case", cases::get_routes())
        .mount("/case-action", case_actions::get_routes())
        .mount("/person", persons::get_routes())
        .mount("/person-job", person_jobs::get_routes())
        .mount("/person-skill", person_skills::get_routes())
        .mount("/person-requirement", person_requirements::get_routes())
        .attach(Db::fairing())
        .attach(cors::cors_fairing());

    rocket.ignite().await?.launch().await
}
