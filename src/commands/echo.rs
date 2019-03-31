use super::Command;
use crate::error::Error;
use crate::respond::Message;

fn run(input: &str) -> Result<Option<Message>, Error> {
    Ok(input.find(' ').map(|index| input[index + 1..].into()))
}

pub fn echo() -> Command {
    Command::new("echo", run)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_empty() {
        assert_eq!(None, run("").unwrap());
    }

    #[test]
    fn test_run_one_word() {
        assert_eq!(None, run("abc").unwrap());
    }

    #[test]
    fn test_run_three_words() {
        assert_eq!(Some("def ghi".into()), run("abc def ghi").unwrap());
    }
}
