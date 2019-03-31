#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket_contrib::json::Json;

use serde::Deserialize;

mod error;

mod respond;
use respond::*;

mod post;
use post::post;

#[derive(Deserialize)]
struct Attachment;

#[post("/", data = "<request>")]
fn respond(request: Json<Request>) {
    match try_respond(request.into_inner()) {
        Ok(Some(message)) => post(message),
        Ok(None) => Ok(()),
        Err(e) => Err(e),
    }.unwrap()
}

fn port() -> u16 {
    std::env::var("PORT")
        .map(|p| p.parse().unwrap())
        .unwrap_or(5000)
}

fn try_main() -> Result<(), rocket::config::ConfigError> {
    let environment = rocket::config::Environment::active()?;
    let config = rocket::config::Config::build(environment)
        .port(port())
        .unwrap();
    rocket::custom(config).mount("/", routes![respond]).launch();
    Ok(())
}

fn main() {
    try_main().unwrap();
}
