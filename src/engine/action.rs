use super::file;
use super::mode;
use super::plan;
use super::plan::Prune;
use crate::config;
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

    pub fn run(&self, ctx: &config::context::Context, path: &String) -> Result<()> {
        let mut f = file::File::load(path)?;
        while let Some(plans) = plan::Plans::parse(f.lines())?.prune(ctx)? {
            if plans.all(|p| p.command().is_error()) {
                plans.iter().for_each(|p| {
                    println!("{p:?}");
                });
                break;
            }

            let p = plans.first()?;
            match p.command() {
                model::Command::Delete => {
                    f.drain(p.begin(), p.end());
                }
                model::Command::Error => {}
            }
        }
        match self.mode {
            mode::Mode::Plan => f.dump(file::Destination::Stdout)?,
            mode::Mode::Check => {
                todo!() //TODO: check if any changes or errors are detected
            }
            // mode::Mode::Apply => f.dump(file::Destination::Overwrite)?,
            mode::Mode::Apply => f.dump(file::Destination::File(path.to_owned()))?,
        };
        Ok(())
    }
}
