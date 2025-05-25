// SPDX-License-Identifier: MPL-2.0
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
        match cli.mode {
            config::cli::Mode::Plan | config::cli::Mode::Check | config::cli::Mode::Apply => {
                match engine::Engine::init(cli)?.run()? {
                    engine::Status::Success => {}
                    engine::Status::Failure => {
                        std::process::exit(1);
                    }
                }
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
