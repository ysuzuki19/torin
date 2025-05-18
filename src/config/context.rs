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
}
