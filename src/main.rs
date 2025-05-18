mod action;
mod config;
mod convert;
mod engine;
mod ext;
mod model;
mod prelude;

fn main() {
    localtrace::with_trace(|| {
        let ctx = config::context::Context::load(vec!["refactoring"]);
        println!("{:?}", ctx);
        let sample_path = "src/e2e/sample.rs";
        let mut f = engine::file::File::load(sample_path)?;
        println!("Loaded file: {:#?}", f);
        while let Some(ops) = action::Actions::parse(f.lines())? {
            let ops = ops.prune(&ctx)?;
            if ops.all(|o| o.command().is_error()) {
                println!("All operations are errors, exiting.");
                break;
            }
            let op = ops.first()?;

            op.apply(&mut f)?;
            // f.dump(model::DumpDestination::Stdout)?;
            // f.dump(engine::file::Destination::Overwrite)?;
            f.dump(engine::file::Destination::File(
                "src/e2e/sample.rs.expected".to_string(),
            ))?;
        }
        Ok(())
    })
}
