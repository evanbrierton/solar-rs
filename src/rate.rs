use core::fmt::Display;

use chrono::{DateTime, Timelike, Utc};

use crate::rate_version::RateVersion;

#[derive(Debug, PartialEq, Eq)]

pub(crate) enum Rate {
    Day,
    Night,
    NightBoost,
    FeedIn,
}

impl Rate {
    fn value(&self, date: DateTime<Utc>) -> f64 {
        (match date.into() {
            RateVersion::V0 => match *self {
                Rate::Day => 0.4008_f64,
                Rate::Night => 0.2092_f64,
                Rate::NightBoost => 0.1228_f64,
                Rate::FeedIn => 0.21_f64,
            },
            RateVersion::V1 => match *self {
                Rate::Day => 0.3615_f64,
                Rate::Night => 0.1783_f64,
                Rate::NightBoost => 0.1047_f64,
                Rate::FeedIn => 0.21_f64,
            },
        } / 1000_f64)
    }

    #[must_use]
    pub fn cost(&self, power: i32, date: DateTime<Utc>) -> f64 {
        f64::from(power) * self.value(date)
    }
}

impl From<DateTime<Utc>> for Rate {
    fn from(date: DateTime<Utc>) -> Self {
        #![allow(clippy::unreachable)]
        match date.hour() {
            23 | 0..=1 | 4..=7 => Rate::Night,
            2..=3 => Rate::NightBoost,
            8..=22 => Rate::Day,
            _ => unreachable!("Invalid hour"),
        }
    }
}

impl Display for Rate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Rate::Day => write!(f, "Day"),
            Rate::Night => write!(f, "Night"),
            Rate::NightBoost => write!(f, "Night Boost"),
            Rate::FeedIn => write!(f, "Feed In"),
        }
    }
}
