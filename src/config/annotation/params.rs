// SPDX-License-Identifier: MPL-2.0
use crate::prelude::*;

pub struct Params {
    pub rule: Option<String>,
    pub date: Option<String>,
}

impl TryFrom<Vec<&str>> for Params {
    type Error = Error;

    fn try_from(parts: Vec<&str>) -> Result<Self> {
        let mut params = Params {
            rule: None,
            date: None,
        };
        for part in parts {
            let [k, v] = part.sized_split::<2>("=")?;
            match k {
                "rule" => {
                    if params.rule.is_some() {
                        trace!("parameter `rule` definition is duplicated")?;
                    } else {
                        params.rule = Some(v.to_string());
                    }
                }
                "date" => {
                    if params.date.is_some() {
                        trace!("parameter `date` definition is duplicated")?;
                    } else {
                        params.date = Some(v.to_string());
                    }
                }
                _ => trace!("Unknown parameter: {}", k)?,
            }
        }
        Ok(params)
    }
}
