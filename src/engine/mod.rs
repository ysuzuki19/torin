pub mod action;
mod file;
mod mode;
mod plan;

use crate::config;
use crate::prelude::*;

pub struct Engine {
    mode: mode::Mode,
    ctx: config::context::Context,
    sources: Vec<String>,
}

impl Engine {
    pub fn init(cli: config::cli::Cli) -> Result<Self> {
        let mode = cli.mode.try_into()?;
        let manifest = config::manifest::Manifest::load()?;
        // torin DELETE BEGIN rule=manifest-execution
        println!("Manifest.project.includes: {:?}", manifest.project.includes);
        println!("Manifest.project.excludes: {:?}", manifest.project.excludes);
        println!("Manifest.project.rules: {:?}", manifest.project.rules);
        println!("Manifest.project.sources: {:?}", manifest.sources()?);
        // torin DELETE END
        let sources = manifest.sources()?;
        let ctx = config::context::Context::load(manifest.project.rules);
        Ok(Self { mode, ctx, sources })
    }

    pub fn run(&self) -> Result<()> {
        for source in &self.sources {
            action::Action::new(self.mode).run(&self.ctx, source)?;
        }
        Ok(())
    }
}
