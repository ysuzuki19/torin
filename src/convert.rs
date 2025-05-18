use crate::{config, operation};

impl From<config::Command> for operation::Command {
    fn from(cmd: config::Command) -> Self {
        match cmd {
            config::Command::Delete => operation::Command::Delete,
            config::Command::Error => operation::Command::Error,
        }
    }
}

impl From<config::Trigger> for operation::Trigger {
    fn from(trigger: config::Trigger) -> Self {
        match trigger {
            config::Trigger::Feature(feature) => operation::Trigger::Feature(feature),
            config::Trigger::Date(date) => operation::Trigger::Date(date),
        }
    }
}
