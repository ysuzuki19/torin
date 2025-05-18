use super::params::Params;
use crate::model::{Date, Feature};
use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Trigger {
    Feature(Feature),
    Date(Date),
}

impl TryFrom<Params> for Option<Trigger> {
    type Error = Error;

    fn try_from(params: Params) -> Result<Self> {
        match (params.feature, params.date) {
            (Some(feature), None) => Ok(Some(Trigger::Feature(Feature::new(feature)))),
            (None, Some(date)) => Ok(Some(Trigger::Date(Date::try_from(date)?))),
            (Some(feature), Some(date)) => trace!(
                "Only one trigger type is allowed (`feature`={}, `date`={})",
                feature, date
            )?,
            (None, None) => Ok(None),
        }
    }
}
