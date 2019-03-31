use super::Command;
use crate::error::Error;
use crate::respond::Message;
use regex::Regex;

fn run(input: &str) -> Result<Option<Message>, Error> {
    Ok(Some(input[5..].into()))
}

pub fn echo() -> Command {
    Command {
        regex: Regex::new("echo ").unwrap(),
        run: Box::new(run),
    }
}
