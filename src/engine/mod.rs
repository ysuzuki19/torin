mod file;
mod mode;
mod plan;
use plan::Prune;

use crate::config;
use crate::model;
use crate::prelude::*;

pub struct Engine {
    mode: mode::Mode,
}

// enum Output {
//     Json,
//     Stdout,
// }
// struct Args {
//     output: Output,
// }

impl Engine {
    pub fn new(mode: mode::Mode) -> Self {
        Engine { mode }
    }

    pub fn run(&self, ctx: config::context::Context, path: String) -> Result<()> {
        println!("Engine is running!");
        // 1. List all files in the directory
        // 2. walk through each file
        let mut f = file::File::load(&path)?;
        while let Some(plans) = plan::Plans::parse(f.lines())?.prune(&ctx)? {
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
            mode::Mode::Apply => f.dump(file::Destination::File(format!("{path}.expected")))?,
        };
        Ok(())
    }
}
