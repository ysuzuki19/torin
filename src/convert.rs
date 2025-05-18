use super::prelude::*;

use crate::{
    action::{self},
    config::annotation::*,
    model,
};

impl From<Command> for action::Command {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::Delete => action::Command::Delete,
            Command::Error => action::Command::Error,
        }
    }
}

impl TryFrom<Params> for Option<model::Trigger> {
    type Error = Error;

    fn try_from(params: Params) -> Result<Self> {
        match (params.feature, params.date) {
            (Some(feature), None) => {
                Ok(Some(model::Trigger::Feature(model::Feature::new(feature))))
            }
            (None, Some(date)) => Ok(Some(model::Trigger::Date(model::Date::try_from(date)?))),
            (Some(feature), Some(date)) => trace!(
                "Only one trigger type is allowed (`feature`={}, `date`={})",
                feature, date
            )?,
            (None, None) => Ok(None),
        }
    }
}
