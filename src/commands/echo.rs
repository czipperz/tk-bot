use super::Command;
use crate::error::Error;
use crate::respond::Message;

fn run(input: &str) -> Result<Option<Message>, Error> {
    Ok(input.find(' ').map(|index| input[index..].into()))
}

pub fn echo() -> Command {
    Command::new("echo", run)
}
