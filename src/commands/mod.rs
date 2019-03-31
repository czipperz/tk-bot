use crate::error::Error;
use crate::respond::Message;
use regex::Regex;

pub struct Command {
    pub regex: Regex,
    pub run: Box<Fn(&str) -> Result<Option<Message>, Error>>,
}

impl Command {
    pub fn new(
        pattern: &str,
        run: impl 'static + Fn(&str) -> Result<Option<Message>, Error>,
    ) -> Self {
        Command {
            regex: Regex::new(pattern).unwrap(),
            run: Box::new(run),
        }
    }
}

mod echo;
mod hello_tk;
pub fn commands() -> Vec<Command> {
    vec![hello_tk::hello_tk(), echo::echo()]
}
