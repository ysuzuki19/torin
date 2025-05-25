pub mod action;
mod context;
mod file;
mod mode;
mod plan;
mod status;

use crate::config;
use crate::prelude::*;
pub use status::Status;

pub struct Engine {
    mode: mode::Mode,
    ctx: context::Context,
    sources: Vec<String>,
}

impl Engine {
    pub fn init(cli: config::cli::Cli) -> Result<Self> {
        let mode = cli.mode.try_into()?;
        let manifest = config::manifest::Manifest::load()?;
        let sources = manifest.sources()?;
        let ctx = context::Context::load(manifest.project.rules);
        Ok(Self { mode, ctx, sources })
    }

    pub fn run(&self) -> Result<Status> {
        let mut status = Status::Success;
        for source in &self.sources {
            match action::Action::new(self.mode).run(&self.ctx, source)? {
                Status::Success => {}
                Status::Failure => {
                    status = Status::Failure;
                }
            }
        }
        Ok(status)
    }
}
