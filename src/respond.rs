use crate::error::Error;
use crate::commands::commands;
use crate::Attachment;
use serde::{Deserialize, Serialize};

pub fn try_respond(request: Request) -> Result<Option<Message>, Error> {
    for command in commands() {
        if command.regex.is_match(&request.text) {
            return (command.run)(&request.text);
        }
    }
    Ok(None)
}

#[derive(Deserialize)]
pub struct Request {
    attachments: Vec<Attachment>,
    avatar_url: Option<String>,
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

#[derive(Debug, Serialize, PartialEq)]
pub struct Message {
    text: String,
}

impl<S: ToString> From<S> for Message {
    fn from(text: S) -> Message {
        Message {
            text: text.to_string(),
        }
    }
}
