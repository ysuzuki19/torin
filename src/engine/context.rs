// SPDX-License-Identifier: MPL-2.0
use crate::model;

#[derive(Debug, PartialEq)]
pub struct Context {
    pub date: model::Date,
    rules: Vec<model::Rule>,
}

impl Context {
    pub fn load(rules: Vec<impl Into<model::Rule>>) -> Self {
        Context {
            date: model::Date::now(),
            rules: rules.into_iter().map(Into::into).collect(),
        }
    }

    pub fn is_triggered(&self, trigger: &model::Trigger) -> bool {
        match trigger {
            model::Trigger::Date(date) => date <= &self.date,
            model::Trigger::Rule(rule) => !self.rules.contains(rule),
        }
    }

    #[cfg(test)]
    pub fn mock(date: model::Date, rules: Vec<impl Into<model::Rule>>) -> Self {
        let mut ctx = Context::load(rules);
        ctx.date = date;
        ctx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context() {
        let ctx = Context::mock(model::Date::mock(2025, 5, 20), vec!["foo", "bar"]);
        assert!(!ctx.is_triggered(&model::Trigger::rule("foo")));
        assert!(ctx.is_triggered(&model::Trigger::rule("baz")));
        assert!(ctx.is_triggered(&model::Trigger::Date(model::Date::mock(2025, 5, 19))));
        assert!(ctx.is_triggered(&model::Trigger::Date(model::Date::mock(2025, 5, 20))));
        assert!(!ctx.is_triggered(&model::Trigger::Date(model::Date::mock(2025, 5, 21))));
    }
}
