use chrono::Datelike;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Date {
    year: i32,
    month: u32,
    day: u32,
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{:02}-{:02}", self.year, self.month, self.day)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_display() {
        let date = Date::new(2023, 10, 5);
        assert_eq!(date.to_string(), "2023-10-05");
    }

    #[test]
    fn test_date_try_from_str() {
        let date: Date = "2023-10-05".try_into().unwrap();
        assert_eq!(date, Date::new(2023, 10, 5));
    }

    #[test]
    fn test_date_now() {
        let now = Date::now();
        assert!(now.year > 2000); // Assuming the test is run after the year 2000
    }

    #[test]
    fn test_date_mock() {
        let date = Date::mock(2023, 10, 5);
        assert_eq!(date, Date::new(2023, 10, 5));
    }

    #[test]
    fn test_compare_dates() {
        let d = Date::new(2025, 10, 5);
        struct Case {
            d: Date,
            less: bool,
            less_equal: bool,
            equal: bool,
        }
        let cases = [
            Case {
                d: Date::new(2024, 10, 5),
                less: true,
                less_equal: true,
                equal: false,
            },
            Case {
                d: Date::new(2025, 9, 5),
                less: true,
                less_equal: true,
                equal: false,
            },
            Case {
                d: Date::new(2025, 10, 4),
                less: true,
                less_equal: true,
                equal: false,
            },
            Case {
                d: Date::new(2025, 10, 5),
                less: false,
                less_equal: true,
                equal: true,
            },
            Case {
                d: Date::new(2025, 10, 6),
                less: false,
                less_equal: false,
                equal: false,
            },
            Case {
                d: Date::new(2025, 11, 5),
                less: false,
                less_equal: false,
                equal: false,
            },
            Case {
                d: Date::new(2026, 10, 5),
                less: false,
                less_equal: false,
                equal: false,
            },
        ];
        for case in cases {
            assert_eq!(case.d < d, case.less, "{} < {} failed", d, case.d);
            assert_eq!(case.d <= d, case.less_equal, "{} <= {} failed", d, case.d);
            assert_eq!(case.d == d, case.equal, "{} == {} failed", d, case.d);
            assert_eq!(case.d > d, !case.less_equal, "{} > {} failed", d, case.d);
            assert_eq!(case.d >= d, !case.less, "{} >= {} failed", d, case.d);
            assert_eq!(case.d != d, !case.equal, "{} != {} failed", d, case.d);
        }
    }
}
