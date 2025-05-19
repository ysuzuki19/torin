use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Command {
    Delete,
    Error,
    // Add UnComment,
}

impl Command {
    pub fn is_error(&self) -> bool {
        matches!(self, Command::Error)
    }
}

impl TryFrom<&str> for Command {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        match s {
            "DELETE" => Ok(Command::Delete),
            "ERROR" => Ok(Command::Error),
            _ => trace!("Invalid command"),
        }
    }
}
