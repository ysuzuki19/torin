// SPDX-License-Identifier: MPL-2.0
use crate::prelude::*;

use super::Trigger;

#[derive(Debug, PartialEq)]
pub enum Target {
    Begin(Trigger),
    End,
    Neighbor(Trigger),
}

impl Target {
    pub fn new(target_mode: &str, trigger: Option<Trigger>) -> Result<Self> {
        match (target_mode, trigger) {
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
