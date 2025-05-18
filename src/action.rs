use crate::engine::file::File;
use crate::prelude::*;
use crate::{config, model};

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
pub struct Action {
    command: Command,
    trigger: model::Trigger,
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
    actions: Vec<Action>,
}

impl Actions {
    pub fn parse(lines: &[String]) -> Result<Option<Self>> {
        let actions: Vec<Action> = lines
            .iter()
            .enumerate()
            .filter(|(_, line)| config::annotation::Annotation::is_match(line))
            .map(|(index, line)| {
                let cfg = config::annotation::Annotation::parse(line)?;
                match cfg.target {
                    config::annotation::Target::Begin(trigger) => Ok(Some(Action {
                        command: cfg.command.into(),
                        trigger,
                        range: Range {
                            begin: index,
                            end: lines
                                .next_match(index, config::annotation::Annotation::is_target_end)
                                .unwrap_or(lines.len() - 1),
                        },
                    })),
                    config::annotation::Target::End => Ok(None),
                    config::annotation::Target::Neighbor(trigger) => Ok(Some(Action {
                        command: cfg.command.into(),
                        trigger,
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

        if actions.is_empty() {
            return Ok(None);
        }
        Ok(Some(Self { actions }))
    }

    pub fn prune(mut self, ctx: &config::context::Context) -> Result<Self> {
        self.actions.retain(|a| ctx.is_triggered(&a.trigger));
        Ok(self)
    }

    pub fn all(&self, predicates: impl Fn(&Action) -> bool) -> bool {
        self.actions.iter().all(predicates)
    }

    pub fn first(&self) -> Result<&Action> {
        match self.actions.first() {
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
                        trigger: model::Trigger::Feature(model::Feature::from("foo")),
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
                        trigger: model::Trigger::Feature(model::Feature::from("foo")),
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
                    assert_eq!(ops.unwrap().actions, case.expected);
                }
            }
            Ok(())
        });
    }
}
