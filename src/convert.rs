use crate::{action, config};

impl From<config::Command> for action::Command {
    fn from(cmd: config::Command) -> Self {
        match cmd {
            config::Command::Delete => action::Command::Delete,
            config::Command::Error => action::Command::Error,
        }
    }
}

impl From<config::Trigger> for action::Trigger {
    fn from(trigger: config::Trigger) -> Self {
        match trigger {
            config::Trigger::Feature(feature) => action::Trigger::Feature(feature),
            config::Trigger::Date(date) => action::Trigger::Date(date),
        }
    }
}
