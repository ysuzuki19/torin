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
        let manifest: Manifest = toml::from_str(&content)?;

        Ok(manifest)
    }
}
