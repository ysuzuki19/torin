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
        match cli.mode {
            config::cli::Mode::Plan | config::cli::Mode::Check | config::cli::Mode::Apply => {
                let ctx = config::context::Context::load(vec!["debug"]);
                let sample_path = "src/e2e/sample.rs".into();
                engine::Engine::new(cli.mode.try_into()?).run(ctx, sample_path)?;
            }
            config::cli::Mode::Completion { shell } => {
                let mut cmd = <config::cli::Cli as clap::CommandFactory>::command();
                let bin_name = cmd.get_name().to_owned();
                clap_complete::generate(shell, &mut cmd, bin_name, &mut std::io::stdout());
            }
        }
        Ok(())
    })
}
