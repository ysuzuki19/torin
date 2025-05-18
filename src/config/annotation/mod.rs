mod params;
mod target;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::{model, prelude::*};
pub use params::Params;
pub use target::Target;

#[derive(Debug, PartialEq)]
pub struct Annotation {
    pub command: model::Command,
    pub target: Target,
}

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*// torin ").expect("Invalid regex"));
impl Annotation {
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
        let trigger = Option::<model::Trigger>::try_from(params)?;

        Ok(Self {
            command: command.try_into()?,
            target: (target_mode, trigger).try_into()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{Date, Feature};

    use super::*;

    #[test]
    fn test_in_code_config() {
        testing::with_trace(|| {
            struct Case {
                input: &'static str,
                expected: std::result::Result<Annotation, ()>,
            }
            let cases = vec![
                Case {
                    input: "// torin DELETE BEGIN feature=foo",
                    expected: Ok(Annotation {
                        command: model::Command::Delete,
                        target: Target::Begin(model::Trigger::Feature(Feature::new("foo"))),
                    }),
                },
                Case {
                    input: "// torin ERROR END",
                    expected: Ok(Annotation {
                        command: model::Command::Error,
                        target: Target::End,
                    }),
                },
                Case {
                    input: "// torin ERROR END feature=foo",
                    expected: Err(()),
                },
                Case {
                    input: "// torin DELETE NEIGHBOR feature=bar",
                    expected: Ok(Annotation {
                        command: model::Command::Delete,
                        target: Target::Neighbor(model::Trigger::Feature(Feature::new("bar"))),
                    }),
                },
                Case {
                    input: "// torin DELETE BEGIN date=2023-10-01",
                    expected: Ok(Annotation {
                        command: model::Command::Delete,
                        target: Target::Begin(model::Trigger::Date(Date::new(2023, 10, 1))),
                    }),
                },
            ];
            for case in cases {
                let got = Annotation::parse(case.input);
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
