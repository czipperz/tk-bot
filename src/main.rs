#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket_contrib::json::Json;

use serde::{Deserialize, Serialize};

mod error;
use error::Error;

#[derive(Deserialize)]
struct Attachment;

#[derive(Deserialize)]
struct Request {
    attachments: Vec<Attachment>,
    avatar_url: String,
    created_at: i32,
    group_id: String,
    id: String,
    name: String,
    sender_id: String,
    sender_type: String,
    source_guid: String,
    system: bool,
    text: String,
    user_id: String,
}

#[post("/", data = "<request>")]
fn respond(request: Json<Request>) {
    try_respond(request.into_inner()).unwrap();
}

fn try_respond(request: Request) -> Result<(), Error> {
    if request.text == "Hello TK" {
        post("ABC".into())
    } else {
        Ok(())
    }
}

#[derive(Debug, Serialize)]
struct Message {
    text: String,
}

impl<S: ToString> From<S> for Message {
    fn from(text: S) -> Message {
        Message {
            text: text.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
struct IDMessage {
    bot_id: String,
    #[serde(flatten)]
    message: Message,
}

fn bot_id() -> Result<String, std::env::VarError> {
    std::env::var("BOT_ID")
}

fn post(message: Message) -> Result<(), Error> {
    let message = IDMessage {
        bot_id: bot_id()?,
        message,
    };
    println!("Posting {:?}", message);
    reqwest::Client::new()
        .post("https://api.groupme.com/v3/bots/post")
        .json(&message)
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
