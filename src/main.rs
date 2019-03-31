#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::Data;

use serde::Serialize;

mod error;
use error::Error;

#[post("/", data = "<request>")]
fn respond(request: Data) {
    try_respond(request).unwrap();
}

fn try_respond(request: Data) -> Result<(), Error> {
    use std::io::Read;
    let mut text = String::new();
    request.open().read_to_string(&mut text)?;
    post(text)
}

#[derive(Serialize)]
struct Message {
    bot_id: String,
    text: String,
}

fn bot_id() -> Result<String, std::env::VarError> {
    std::env::var("BOT_ID")
}

fn post(text: String) -> Result<(), Error> {
    reqwest::Client::new()
        .post("api.groupme.com/v3/bots/post")
        .json(&Message {
            bot_id: bot_id()?,
            text,
        })
        .send()?;
    Ok(())
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
