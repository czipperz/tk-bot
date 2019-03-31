use super::Command;
use crate::error::Error;
use crate::respond::Message;

fn run(input: &str) -> Result<Option<Message>, Error> {
    Ok(Some(input[5..].into()))
}

pub fn echo() -> Command {
    Command::new("echo", run)
}
