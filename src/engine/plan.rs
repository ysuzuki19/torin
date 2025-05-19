use crate::engine::file::File;
use crate::prelude::*;
use crate::{config, model};

#[derive(Debug, PartialEq)]
pub struct Range {
    pub begin: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq)]
pub struct Plan {
    command: model::Command,
    trigger: model::Trigger,
    range: Range,
}

impl Plan {
    // torin DELETE BEGIN feature=git-init
    // pub fn new(command: Command, begin: usize, end: usize) -> Self {
    //     Self {
    //         command,
    //         range: Range { begin, end },
    //     }
    // }
    // torin DELETE END

    pub fn command(&self) -> &model::Command {
        &self.command
    }

    pub fn apply(&self, f: &mut File) -> Result<()> {
        match self.command {
            model::Command::Delete => {
                f.drain(self.range.begin, self.range.end);
                Ok(())
            }
            model::Command::Error => Ok(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Plans {
    plans: Vec<Plan>,
}

impl Plans {
    pub fn parse(lines: &[String]) -> Result<Option<Self>> {
        let plans: Vec<Plan> = lines
            .iter()
            .enumerate()
            .filter(|(_, line)| config::annotation::Annotation::is_match(line))
            .map(|(index, line)| {
                let cfg = config::annotation::Annotation::parse(line)?;
                match cfg.target {
                    model::Target::Begin(trigger) => {
                        let next_annotation_index = lines
                            .next_match(index + 1, |line| {
                                config::annotation::Annotation::is_match(line)
                            })
                            .unwrap_or(lines.len() - 1);
                        let next_annotation =
                            config::annotation::Annotation::parse(&lines[next_annotation_index])?;

                        let end = match next_annotation.target {
                            model::Target::Begin(_) => {
                                return trace!("Nested torin annotation found: {}", line); //TODO: update error message
                            }
                            model::Target::Neighbor(_) => {
                                return trace!("Nested torin annotation found: {}", line); //TODO: update error message
                            }
                            model::Target::End => next_annotation_index,
                        };
                        Ok(Some(Plan {
                            command: cfg.command,
                            trigger,
                            range: Range { begin: index, end },
                        }))
                    }
                    model::Target::End => Ok(None),
                    model::Target::Neighbor(trigger) => Ok(Some(Plan {
                        command: cfg.command,
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

        if plans.is_empty() {
            return Ok(None);
        }
        Ok(Some(Self { plans }))
    }

    pub fn prune(mut self, ctx: &config::context::Context) -> Result<Self> {
        self.plans.retain(|a| ctx.is_triggered(&a.trigger));
        Ok(self)
    }

    pub fn all(&self, predicates: impl Fn(&Plan) -> bool) -> bool {
        self.plans.iter().all(predicates)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Plan> {
        self.plans.iter()
    }

    pub fn first(&self) -> Result<&Plan> {
        match self.plans.first() {
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
                expected: Vec<Plan>,
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
                    expected: vec![Plan {
                        command: model::Command::Delete,
                        trigger: model::Trigger::Feature(model::Feature::from("foo")),
                        range: Range { begin: 3, end: 5 },
                    }],
                },
                Case {
                    lines: vec![
                        String::from("fuga"),
                        String::from("// torin DELETE BEGIN feature=foo"),
                        String::from("// torin DELETE END"),
                        String::from("hoge"),
                    ],
                    expected: vec![Plan {
                        command: model::Command::Delete,
                        trigger: model::Trigger::Feature(model::Feature::from("foo")),
                        range: Range { begin: 1, end: 2 },
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
                    expected: vec![Plan {
                        command: model::Command::Delete,
                        trigger: model::Trigger::Feature(model::Feature::from("foo")),
                        range: Range { begin: 4, end: 6 },
                    }],
                },
            ];
            for case in cases {
                let ops = Plans::parse(&case.lines)?;
                if case.expected.is_empty() {
                    assert!(ops.is_none(), "Expected no operations, got: {:?}", ops);
                } else {
                    assert!(ops.is_some(), "Expected operations, got None");
                    assert_eq!(ops.unwrap().plans, case.expected);
                }
            }
            Ok(())
        });
    }
}
