mod config;
mod convert;
mod engine;
mod ext;
mod model;
mod operation;
mod prelude;

use localtrace::*;
use model::*;

fn main() {
    with_trace(|| {
        // std::env::set_var("", value);
        println!("{}", Date::now()?);
        let sample_path = "sample.rs";
        let mut f = engine::file::File::load(sample_path)?;
        println!("Loaded file: {:#?}", f);
        while let Some(ops) = operation::Operations::parse(f.lines())? {
            let ops = ops.prune()?;
            if ops.all(|o| o.command().is_error()) {
                println!("All operations are errors, exiting.");
                break;
            }
            let op = ops.first()?;

            op.apply(&mut f)?;
            // f.dump(model::DumpDestination::Stdout)?;
            // f.dump(engine::file::Destination::Overwrite)?;
            f.dump(engine::file::Destination::File("sample.out.rs".to_string()))?;
        }
        Ok(())
    })
}
