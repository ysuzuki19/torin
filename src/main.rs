mod config;
mod convert;
mod engine;
mod ext;
mod model;
mod prelude;

fn main() {
    // 1. parse cli arguments
    // 2. load .torin.yml and additional config files

    localtrace::with_trace(|| {
        let ctx = config::context::Context::load(vec!["debug"]);
        let sample_path = "src/e2e/sample.rs".into();
        engine::Engine::new(engine::Mode::Plan).run(ctx, sample_path)?;
        Ok(())
    })
}
