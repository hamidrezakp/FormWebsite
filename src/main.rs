#![allow(clippy::trivial_regex)]
#![allow(dead_code)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate frunk;

mod bot;
mod dialogue;
mod errors;
mod models;
mod schema;
mod website;

#[tokio::main]
async fn main() {
    tokio::spawn(website::run());
    bot::run().await;
}
