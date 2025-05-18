use once_cell::sync::Lazy;
use regex::Regex;

use crate::model::{Date, Feature};
use crate::prelude::*;

pub struct Params {
    feature: Option<String>,
    date: Option<String>,
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

#[derive(Debug, PartialEq)]
pub enum Command {
    Delete,
    Error,
}

impl TryFrom<&str> for Command {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        match s {
            "DELETE" => Ok(Command::Delete),
            "ERROR" => Ok(Command::Error),
            _ => trace!("Invalid command"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Target {
    Begin(Trigger),
    End,
    Neighbor(Trigger),
}

impl TryFrom<(&str, Option<Trigger>)> for Target {
    type Error = Error;

    fn try_from((s, trigger): (&str, Option<Trigger>)) -> Result<Self> {
        match (s, trigger) {
            ("BEGIN", Some(trigger)) => Ok(Self::Begin(trigger)),
            ("BEGIN", None) => trace!("Trigger is required for BEGIN mode"),

            ("END", Some(_)) => trace!("Trigger is not allowed for END mode"),
            ("END", None) => Ok(Self::End),

            ("NEIGHBOR", Some(trigger)) => Ok(Self::Neighbor(trigger)),
            ("NEIGHBOR", None) => trace!("Trigger is required for NEIGHBOR mode"),

            _ => trace!("Invalid target mode"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct InCodeConfig {
    pub command: Command,
    pub target: Target,
}

static RE: Lazy<regex::Regex> = Lazy::new(|| Regex::new(r"^\s*// torin ").expect("Invalid regex"));
impl InCodeConfig {
    pub fn is_match(line: &str) -> bool {
        RE.is_match(line)
    }

    pub fn is_target_end<S>(line: &S) -> bool
    where
        S: AsRef<str>,
    {
        if let Ok(cfg) = Self::parse(line.as_ref()) {
            matches!(cfg.target, Target::End)
        } else {
            false
        }
    }

    pub fn parse(line: &str) -> Result<Self> {
        if !Self::is_match(line) {
            return trace!("Invalid line");
        }

        let ([command, target_mode], rest) = line
            .trim()
            .trim_start_matches("// torin ")
            .least_sized_split::<2>(" ")?;
        let params = Params::try_from(rest)?;
        let trigger = Option::<Trigger>::try_from(params)?;

        Ok(Self {
            command: Command::try_from(command)?,
            target: Target::try_from((target_mode, trigger))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_code_config() {
        testing::with_trace(|| {
            struct Case {
                input: &'static str,
                expected: std::result::Result<InCodeConfig, ()>,
            }
            let cases = vec![
                Case {
                    input: "// torin DELETE BEGIN feature=foo",
                    expected: Ok(InCodeConfig {
                        command: Command::Delete,
                        target: Target::Begin(Trigger::Feature(Feature::new("foo"))),
                    }),
                },
                Case {
                    input: "// torin ERROR END",
                    expected: Ok(InCodeConfig {
                        command: Command::Error,
                        target: Target::End,
                    }),
                },
                Case {
                    input: "// torin ERROR END feature=foo",
                    expected: Err(()),
                },
                Case {
                    input: "// torin DELETE NEIGHBOR feature=bar",
                    expected: Ok(InCodeConfig {
                        command: Command::Delete,
                        target: Target::Neighbor(Trigger::Feature(Feature::new("bar"))),
                    }),
                },
                Case {
                    input: "// torin DELETE BEGIN date=2023-10-01",
                    expected: Ok(InCodeConfig {
                        command: Command::Delete,
                        target: Target::Begin(Trigger::Date(Date::new(2023, 10, 1))),
                    }),
                },
            ];
            for case in cases {
                let got = InCodeConfig::parse(case.input);
                match case.expected {
                    Ok(expected) => {
                        assert!(got.is_ok(), "Failed to parse: `{}`", case.input);
                        assert_eq!(got?, expected);
                    }
                    Err(_) => assert!(got.is_err(), "Expected error for input: `{}`", case.input),
                }
            }
            Ok(())
        });
    }
}
