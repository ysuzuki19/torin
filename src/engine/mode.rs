use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Plan,
    Apply,
}

impl TryFrom<&str> for Mode {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "plan" => Ok(Mode::Plan),
            "apply" => Ok(Mode::Apply),
            _ => Err("Invalid mode, expected 'plan' or 'apply'"),
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::Plan => write!(f, "plan"),
            Mode::Apply => write!(f, "apply"),
        }
    }
}
