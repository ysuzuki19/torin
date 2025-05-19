use super::*;

#[derive(Debug, PartialEq)]
pub enum Trigger {
    Feature(Feature),
    Date(Date),
}

impl Trigger {
    pub fn feature(feature: impl Into<Feature>) -> Self {
        Trigger::Feature(feature.into())
    }
}
