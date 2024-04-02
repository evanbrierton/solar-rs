use core::str::FromStr;

use chrono::{DateTime, Utc};
use clap::ValueEnum;

#[non_exhaustive]
#[derive(Default, Debug, Clone, Copy, ValueEnum)]
pub enum Period {
    Minute,
    Hour,
    Day,
    #[default]
    Month,
    Year,
}

impl Period {
    #[must_use]
    #[inline]
    pub fn key(&self, date: &DateTime<Utc>) -> String {
        match *self {
            Self::Minute => format!("{}", date.format("%Y-%m-%d %H:%M")),
            Self::Hour => format!("{}", date.format("%Y-%m-%d %H")),
            Self::Day => format!("{}", date.format("%Y-%m-%d")),
            Self::Month => format!("{}", date.format("%Y-%m")),
            Self::Year => format!("{}", date.format("%Y")),
        }
    }
}

impl FromStr for Period {
    type Err = String;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "minute" | "minutes" => Ok(Self::Minute),
            "hour" | "hours" => Ok(Self::Hour),
            "day" | "days" => Ok(Self::Day),
            "month" | "months" => Ok(Self::Month),
            "year" | "years" => Ok(Self::Year),
            _ => Err(format!("Invalid period: {s}")),
        }
    }
}
