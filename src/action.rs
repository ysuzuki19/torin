use crate::config;
use crate::engine::file::File;
use crate::model::Date;
use crate::model::Feature;
use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Command {
    Delete,
    Error,
}

impl Command {
    pub fn is_error(&self) -> bool {
        matches!(self, Command::Error)
    }
}

#[derive(Debug, PartialEq)]
pub struct Range {
    pub begin: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq)]
pub enum Trigger {
    Feature(Feature),
    Date(Date),
}

#[derive(Debug, PartialEq)]
pub struct Action {
    command: Command,
    range: Range,
}

impl Action {
    // torin DELETE BEGIN feature=git-init
    // pub fn new(command: Command, begin: usize, end: usize) -> Self {
    //     Self {
    //         command,
    //         range: Range { begin, end },
    //     }
    // }
    // torin DELETE END

    pub fn command(&self) -> &Command {
        &self.command
    }

    pub fn apply(&self, f: &mut File) -> Result<()> {
        match self.command {
            Command::Delete => {
                f.drain(self.range.begin, self.range.end);
                Ok(())
            }
            Command::Error => Ok(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Actions {
    operations: Vec<Action>,
}

impl Actions {
    pub fn parse(lines: &[String]) -> Result<Option<Self>> {
        let operations: Vec<Action> = lines
            .iter()
            .enumerate()
            .filter(|(_, line)| config::InCodeConfig::is_match(line))
            .map(|(index, line)| {
                let cfg = config::InCodeConfig::parse(line)?;
                match cfg.target {
                    config::Target::Begin(_) => Ok(Some(Action {
                        command: cfg.command.into(),
                        range: Range {
                            begin: index,
                            end: lines
                                .next_match(index, config::InCodeConfig::is_target_end)
                                .unwrap_or(lines.len() - 1),
                        },
                    })),
                    config::Target::End => Ok(None),
                    config::Target::Neighbor(_) => Ok(Some(Action {
                        command: cfg.command.into(),
                        range: Range {
                            begin: lines
                                .prev_match(index, |line| line.trim().is_empty())
                                .map(|matched| {
                                    if lines[matched].trim().is_empty() {
                                        matched + 1
                                    } else {
                                        matched
                                    }
                                })
                                .unwrap_or(0),
                            end: lines
                                .next_match(index, |line| line.trim().is_empty())
                                .map(|matched| {
                                    if lines[matched].trim().is_empty() {
                                        matched - 1
                                    } else {
                                        matched
                                    }
                                })
                                .unwrap_or(lines.len() - 1),
                        },
                    })),
                }
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect();

        if operations.is_empty() {
            return Ok(None);
        }
        Ok(Some(Self { operations }))
    }

    pub fn prune(self) -> Result<Self> {
        //TODO
        // Input {date,features}
        // Output pruned operations by date and features
        // e.g.) on `sample.rs`: // - `torin DELETE BEGIN date=2025-10-01` is removed
        Ok(self)
    }

    pub fn all(&self, predicates: impl Fn(&Action) -> bool) -> bool {
        self.operations.iter().all(predicates)
    }

    pub fn first(&self) -> Result<&Action> {
        match self.operations.first() {
            Some(op) => Ok(op),
            None => trace!("No operations found"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operations() {
        testing::with_trace(|| {
            struct Case {
                lines: Vec<String>,
                expected: Vec<Action>,
            }
            let cases = [
                Case {
                    lines: vec![
                        String::from("fuga"),
                        String::from(""),
                        String::from("hoge"),
                        String::from("// torin DELETE BEGIN feature=foo"),
                        String::from("some code"),
                        String::from("// torin DELETE END"),
                        String::from("fuga"),
                        String::from(""),
                        String::from("hoge"),
                    ],
                    expected: vec![Action {
                        command: Command::Delete,
                        range: Range { begin: 3, end: 5 },
                    }],
                },
                Case {
                    lines: vec![
                        String::from("fuga"),
                        String::from(""),
                        String::from("hoge"),
                        String::from(""),
                        String::from("// torin DELETE NEIGHBOR feature=foo"),
                        String::from("some code"),
                        String::from("fuga"),
                        String::from(""),
                        String::from("hoge"),
                        String::from(""),
                    ],
                    expected: vec![Action {
                        command: Command::Delete,
                        range: Range { begin: 4, end: 6 },
                    }],
                },
            ];
            for case in cases {
                let ops = Actions::parse(&case.lines)?;
                if case.expected.is_empty() {
                    assert!(ops.is_none(), "Expected no operations, got: {:?}", ops);
                } else {
                    assert!(ops.is_some(), "Expected operations, got None");
                    assert_eq!(ops.unwrap().operations, case.expected);
                }
            }
            Ok(())
        });
    }
}
