use super::context;
use super::file;
use super::mode;
use super::plan;
use super::plan::Prune;
use super::Status;
use crate::model;
use crate::model::cutify::CutifyOps;
use crate::prelude::*;

pub struct Action {
    mode: mode::Mode,
}


impl Action {
    pub fn new(mode: mode::Mode) -> Self {
        Action { mode }
    }

    pub fn run(&self, ctx: &context::Context, path: &String) -> Result<Status> {
        let mut f = file::File::load(path)?;
        let mut errors = Option::<Vec<_>>::None;
        while let Some(plans) = plan::Plans::parse(&f.lines())?.prune(ctx)? {
            if plans.all(|p| p.command().is_error()) {
                errors.replace(plans.iter().cloned().collect());
                break;
            }

            let p = plans.first()?;
            match p.command() {
                model::Command::Delete => {
                    f.flagging(p.begin(), p.end());
                }
                model::Command::Error => {}
            }
        }
        match self.mode {
            mode::Mode::Plan => {
                for diff in f.diffs() {
                    println!("{}:{}", path.cutify().bold(), diff.lineno());
                    println!("{}\n", diff.unified_diff_format());
                }
            }
            mode::Mode::Check => {
                let mut succeed = true;
                for diff in f.diffs() {
                    println!("{}:{}", path.cutify().bold(), diff.lineno());
                    println!("{}\n", diff.unified_diff_format());
                    succeed = false;
                }

                if let Some(errors) = errors.take() {
                    for p in errors {
                        println!("check: {}:{}", path.cutify().bold(), p.begin() + 1);
                    }
                    succeed = false;
                }

                if !succeed {
                    return Ok(Status::Failure);
                }
            }
            mode::Mode::Apply => {
                f.apply();
                f.dump(file::Destination::Overwrite)?;
            }
        };
        Ok(Status::Success)
    }
}
