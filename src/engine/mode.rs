use std::fmt;

use crate::config;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Plan,
    Check,
    Apply,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plan => write!(f, "plan"),
            Self::Check => write!(f, "check"),
            Self::Apply => write!(f, "apply"),
        }
    }
}

impl TryFrom<config::cli::Mode> for Mode {
    type Error = Error;
    fn try_from(mode: config::cli::Mode) -> Result<Self> {
        match mode {
            config::cli::Mode::Plan => Ok(Mode::Plan),
            config::cli::Mode::Check => Ok(Mode::Check),
            config::cli::Mode::Apply => Ok(Mode::Apply),
            config::cli::Mode::Completion { .. } => {
                // desired unreachable
                Err(Error::new("Completion mode is not supported"))
            }
        }
    }
}
