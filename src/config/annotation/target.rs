use crate::{model, prelude::*};

#[derive(Debug, PartialEq)]
pub enum Target {
    Begin(model::Trigger),
    End,
    Neighbor(model::Trigger),
}

impl TryFrom<(&str, Option<model::Trigger>)> for Target {
    type Error = Error;

    fn try_from((s, trigger): (&str, Option<model::Trigger>)) -> Result<Self> {
        match (s, trigger) {
            ("BEGIN", Some(trigger)) => Ok(Self::Begin(trigger)),
            ("BEGIN", None) => trace!("Trigger is required for BEGIN mode"),

            ("END", Some(_)) => trace!("Trigger is not allowed for END mode"),
            ("END", None) => Ok(Self::End),

            ("NEIGHBOR", Some(trigger)) => Ok(Self::Neighbor(trigger)),
            ("NEIGHBOR", None) => trace!("Trigger is required for NEIGHBOR mode"),

            _ => trace!("Invalid target mode"),
        }
    }
}
