// SPDX-License-Identifier: MPL-2.0
use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Trigger {
    Rule(Rule),
    Date(Date),
}

impl Trigger {
    pub fn rule(rule: impl Into<Rule>) -> Self {
        Trigger::Rule(rule.into())
    }
}
