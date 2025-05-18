use crate::prelude::*;

pub struct Params {
    pub(super) feature: Option<String>,
    pub(super) date: Option<String>,
}

impl TryFrom<Vec<&str>> for Params {
    type Error = Error;

    fn try_from(parts: Vec<&str>) -> Result<Self> {
        let mut params = Params {
            feature: None,
            date: None,
        };
        for part in parts {
            let [k, v] = part.sized_split::<2>("=")?;
            match k {
                "feature" => {
                    if params.feature.is_some() {
                        trace!("parameter `feature` definition is duplicated")?;
                    } else {
                        params.feature = Some(v.to_string());
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
