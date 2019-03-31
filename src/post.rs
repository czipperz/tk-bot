use crate::error::Error;
use crate::respond::Message;
use serde::Serialize;

pub fn post(message: Message) -> Result<(), Error> {
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

#[derive(Debug, Serialize)]
struct IDMessage {
    bot_id: String,
    #[serde(flatten)]
    message: Message,
}

fn bot_id() -> Result<String, std::env::VarError> {
    std::env::var("BOT_ID")
}
