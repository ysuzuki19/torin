mod config;
mod convert;
mod engine;
mod ext;
mod model;
mod prelude;

use clap::Parser;

fn main() {
    let cli = config::cli::Cli::parse();
    localtrace::with_trace(|| {
        let manifest = config::manifest::Manifest::load()?;
        println!("Manifest.project.includes: {:?}", manifest.project.includes);
        println!("Manifest.project.excludes: {:?}", manifest.project.excludes);
        println!("Manifest.project.rules: {:?}", manifest.project.rules);
        println!("Manifest.project.sources: {:?}", manifest.sources()?);

        match cli.mode {
            config::cli::Mode::Plan | config::cli::Mode::Check | config::cli::Mode::Apply => {
                let ctx = config::context::Context::load(manifest.project.rules);
                let sample_path = "src/e2e/sample.rs".into();
                engine::action::Action::new(cli.mode.try_into()?).run(ctx, sample_path)?;
            }
            config::cli::Mode::Completion { shell } => {
                let mut cmd = <config::cli::Cli as clap::CommandFactory>::command();
                let bin_name = cmd.get_name().to_owned();
                clap_complete::generate(shell, &mut cmd, bin_name, &mut std::io::stdout());
            }
        }
        Ok(())
    });
}
