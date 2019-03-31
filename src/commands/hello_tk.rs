use super::Command;
use crate::error::Error;
use crate::respond::Message;
use regex::Regex;

fn run(input: &str) -> Result<Option<Message>, Error> {
    Ok(Some("ABC".into()))
}

pub fn hello_tk() -> Command {
    Command {
        regex: Regex::new("Hello TK").unwrap(),
        run: Box::new(run),
    }
}
