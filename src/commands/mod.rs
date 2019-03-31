use crate::error::Error;
use crate::respond::Message;
use regex::Regex;

pub struct Command {
    pub regex: Regex,
    pub run: Box<Fn(&str) -> Result<Option<Message>, Error>>,
}

mod echo;
mod hello_tk;
pub fn commands() -> Vec<Command> {
    vec![hello_tk::hello_tk(), echo::echo()]
}
