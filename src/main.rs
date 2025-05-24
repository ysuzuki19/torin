mod config;
mod convert;
mod engine;
mod ext;
mod model;
mod prelude;

use clap::Parser;

fn cli() -> localtrace::Result<()> {
    let cli = config::cli::Cli::parse();
    let manifest = config::manifest::Manifest::load()?;
    println!("Manifest.project.includes: {:?}", manifest.project.includes);
    println!("Manifest.project.excludes: {:?}", manifest.project.excludes);
    println!("Manifest.project.rules: {:?}", manifest.project.rules);
    println!("Manifest.project.sources: {:?}", manifest.sources()?);

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
    });

    Ok(())
}

fn main() {
    let result = cli();
    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
