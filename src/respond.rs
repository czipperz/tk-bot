use crate::error::Error;
use crate::Attachment;
use serde::{Deserialize, Serialize};

pub fn try_respond(request: Request) -> Result<Option<Message>, Error> {
    if request.text == "Hello TK" {
        Ok(Some("ABC".into()))
    } else {
        Ok(None)
    }
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

#[derive(Debug, Serialize)]
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
