use chrono::Datelike;

use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Date {
    year: i32,
    month: u32,
    day: u32,
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day)
    }
}

impl Date {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Date { year, month, day }
    }

    pub fn now() -> Self {
        let now = chrono::Local::now();
        Self::new(now.year(), now.month(), now.day())
    }

    #[cfg(test)]
    pub fn mock(year: i32, month: u32, day: u32) -> Self {
        Date { year, month, day }
    }
}

impl TryFrom<String> for Date {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for Date {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let [year, month, day] = value.sized_split::<3>("-")?;
        Ok(Date {
            year: year.parse()?,
            month: month.parse()?,
            day: day.parse()?,
        })
    }
}
