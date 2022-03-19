#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod errors;
mod models;
mod schema;
mod service_options;
mod website;

#[tokio::main]
async fn main() {
    website::run().await.unwrap();
}
