use super::context;
use super::file;
use super::mode;
use super::plan;
use super::plan::Prune;
use crate::model;
use crate::prelude::*;

pub struct Action {
    mode: mode::Mode,
}

// torin DELETE NEIGHBOR rule=today
// enum Output {
//     Json,
//     Stdout,
// }
// struct Args {
//     output: Output,
// }

impl Action {
    pub fn new(mode: mode::Mode) -> Self {
        Action { mode }
    }

    pub fn run(&self, ctx: &context::Context, path: &String) -> Result<()> {
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
                if let Some(diff) = f.diff() {
                    println!("{diff}\n");
                }
            }
            mode::Mode::Check => {
                let mut succeed = true;
                if let Some(diff) = f.diff() {
                    println!("{diff}\n");
                    succeed = false;
                }

                if let Some(errors) = errors.take() {
                    for e in errors {
                        println!("Error: {e:?}");
                    }
                    succeed = false;
                }

                if !succeed {
                    return Err(Error::new("Check failed: there are changes to apply"));
                }
            }
            mode::Mode::Apply => {
                f.apply();
                f.dump(file::Destination::Overwrite)?;
            }
        };
        Ok(())
    }
}
