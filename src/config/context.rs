use crate::model;

#[derive(Debug, PartialEq)]
pub struct Context {
    pub date: model::Date,
    features: Vec<model::Feature>,
}

impl Context {
    pub fn load(features: Vec<impl Into<model::Feature>>) -> Self {
        Context {
            date: model::Date::now(),
            features: features.into_iter().map(Into::into).collect(),
        }
    }

    pub fn is_triggered(&self, trigger: &model::Trigger) -> bool {
        match trigger {
            model::Trigger::Date(date) => &self.date <= date,
            model::Trigger::Feature(feature) => !self.features.contains(feature),
        }
    }

    #[cfg(test)]
    pub fn mock(date: model::Date, features: Vec<impl Into<model::Feature>>) -> Self {
        let mut ctx = Context::load(features);
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
        assert!(!ctx.is_triggered(&model::Trigger::feature("foo")));
        assert!(ctx.is_triggered(&model::Trigger::feature("baz")));
        assert!(!ctx.is_triggered(&model::Trigger::Date(model::Date::mock(2025, 5, 19))));
        assert!(ctx.is_triggered(&model::Trigger::Date(model::Date::mock(2025, 5, 20))));
        assert!(ctx.is_triggered(&model::Trigger::Date(model::Date::mock(2025, 5, 21))));
    }
}
