use super::prelude::*;

use crate::config;
use crate::model;

impl TryFrom<config::annotation::Params> for Option<model::Trigger> {
    type Error = Error;

    fn try_from(params: config::annotation::Params) -> Result<Self> {
        match (params.rule, params.date) {
            (Some(rule), None) => Ok(Some(model::Trigger::Rule(model::Rule::new(rule)))),
            (None, Some(date)) => Ok(Some(model::Trigger::Date(model::Date::try_from(date)?))),
            (Some(rule), Some(date)) => trace!(
                "Only one trigger type is allowed (`rule`={}, `date`={})",
                rule, date
            )?,
            (None, None) => Ok(None),
        }
    }
}
