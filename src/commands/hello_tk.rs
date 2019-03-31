use super::Command;
use crate::error::Error;
use crate::respond::Message;

fn run(input: &str) -> Result<Option<Message>, Error> {
    Ok(Some("ABC".into()))
}

pub fn hello_tk() -> Command {
    Command::new("Hello TK", run)
}
