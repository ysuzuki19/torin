mod config;
mod convert;
mod engine;
mod ext;
mod model;
mod prelude;

use clap::Parser;

fn main() {
    let cli = config::cli::Cli::parse();
    // 2. load .torin.yml and additional config files

    localtrace::with_trace(|| {
        let ctx = config::context::Context::load(vec!["debug"]);
        let sample_path = "src/e2e/sample.rs".into();
        engine::Engine::new(cli.mode).run(ctx, sample_path)?;
        Ok(())
    })
}
