mod action;
mod file;
mod mode;
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
        // 1. load file
        // 2. parse actions from file
        // 3. apply actions
        // 4. dump file (switch by mode)
        let mut f = file::File::load(&path)?;
        while let Some(ops) = action::Actions::parse(f.lines())? {
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
            f.dump(file::Destination::File(format!("{}.expected", path)))?;
        }
        Ok(())
    }
}
