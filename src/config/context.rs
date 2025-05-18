use crate::model::{Date, Feature};

use super::annotation;

#[derive(Debug, PartialEq)]
pub struct Context {
    pub date: Date,
    features: Vec<Feature>,
}

impl Context {
    pub fn load(features: Vec<impl Into<Feature>>) -> Self {
        Context {
            date: Date::now(),
            features: features.into_iter().map(Into::into).collect(),
        }
    }

    pub fn is_triggered(&self, trigger: annotation::Trigger) -> bool {
        match trigger {
            annotation::Trigger::Date(date) => self.date == date,
            annotation::Trigger::Feature(feature) => self.features.contains(&feature),
        }
    }
}
