use super::Command;
use crate::error::Error;
use crate::respond::Message;

fn run(input: &str) -> Result<Option<Message>, Error> {
    Ok(if let Some(index) = input.find(' ') {
        let trimmed = input[index..].trim_left();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.into())
        }
    } else {
        None
    })
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

    #[test]
    fn test_run_space_at_end() {
        assert_eq!(None, run("abc ").unwrap());
    }
}
