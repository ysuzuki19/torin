use super::*;

#[derive(Debug, PartialEq)]
pub enum Trigger {
    Rule(Rule),
    Date(Date),
}

impl Trigger {
    pub fn rule(rule: impl Into<Rule>) -> Self {
        Trigger::Rule(rule.into())
    }
}
