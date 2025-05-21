use std::fmt;

use crate::config;

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

impl From<config::cli::Mode> for Mode {
    fn from(mode: config::cli::Mode) -> Self {
        match mode {
            config::cli::Mode::Plan => Mode::Plan,
            config::cli::Mode::Check => Mode::Check,
            config::cli::Mode::Apply => Mode::Apply,
        }
    }
}
