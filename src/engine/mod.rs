mod file;
mod mode;
mod plan;
pub use mode::Mode;

use crate::config;
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
        while let Some(ops) = plan::Plans::parse(f.lines())? {
            let ops = ops.prune(&ctx)?;
            if ops.all(|o| o.command().is_error()) {
                ops.iter().for_each(|op| {
                    println!("{:?}", op);
                });
                break;
            }
            let op = ops.first()?;

            println!("Applying operation for {}: {:?}", path, op);
            op.apply(&mut f)?;
            // f.dump(model::DumpDestination::Stdout)?;
            // f.dump(engine::file::Destination::Overwrite)?;
        }
        match self.mode {
            mode::Mode::Plan => f.dump(file::Destination::Stdout)?,
            // mode::Mode::Apply => f.dump(file::Destination::Overwrite)?,
            mode::Mode::Apply => f.dump(file::Destination::File(format!("{}.expected", path)))?,
        };
        Ok(())
    }
}
