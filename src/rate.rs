use chrono::{DateTime, Timelike, Utc};

#[derive(PartialEq, Eq)]

pub enum Rate {
    Day,
    Night,
    NightBoost,
    FeedIn,
}

impl Rate {
    fn value(&self) -> f64 {
        match *self {
            Rate::Day => 0.000_438_6_f64,
            Rate::Night => 0.000_215_5_f64,
            Rate::NightBoost => 0.000_126_5_f64,
            Rate::FeedIn => 0.00021_f64,
        }
    }

    #[must_use]
    pub fn cost(&self, power: i32) -> f64 {
        f64::from(power) * self.value()
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

impl From<Rate> for f64 {
    fn from(rate: Rate) -> Self {
        rate.value()
    }
}
