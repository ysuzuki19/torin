use crate::{
    action::{self},
    config::annotation::*,
};

impl From<Command> for action::Command {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::Delete => action::Command::Delete,
            Command::Error => action::Command::Error,
        }
    }
}

impl From<Trigger> for action::Trigger {
    fn from(trigger: Trigger) -> Self {
        match trigger {
            Trigger::Feature(f) => action::Trigger::Feature(f),
            Trigger::Date(d) => action::Trigger::Date(d),
        }
    }
}
