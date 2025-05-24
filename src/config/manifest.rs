use glob::glob;
use globset::{Glob, GlobSetBuilder};

use crate::prelude::*;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Manifest {
    pub project: Project,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Project {
    pub includes: Vec<String>,
    pub excludes: Vec<String>,
    pub rules: Vec<String>,
}

impl Manifest {
    pub fn load() -> Result<Manifest> {
        let content = std::fs::read_to_string(".torin.toml")?;
        let manifest = toml::from_str(&content)?;

        Ok(manifest)
    }

    pub fn sources(&self) -> Result<Vec<String>> {
        let set = {
            let mut builder = GlobSetBuilder::new();
            for exclude in &self.project.excludes {
                builder.add(Glob::new(exclude)?);
            }
            builder.build()?
        };

        let mut srcs = vec![];
        for include in &self.project.includes {
            glob(include)?
                .flatten()
                .filter(|path| !set.is_match(path))
                .for_each(|path| srcs.push(path.to_string_lossy().to_string()));
        }

        Ok(srcs)
    }
}
