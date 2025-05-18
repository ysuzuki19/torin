use crate::prelude::*;

use super::Trigger;

#[derive(Debug, PartialEq)]
pub enum Target {
    Begin(Trigger),
    End,
    Neighbor(Trigger),
}

impl TryFrom<(&str, Option<Trigger>)> for Target {
    type Error = Error;

    fn try_from((s, trigger): (&str, Option<Trigger>)) -> Result<Self> {
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
