use chrono::{DateTime, Timelike, Utc};

pub enum Rate {
    Day,
    Night,
    NightBoost,
    Purchase,
}

impl Rate {
    fn value(&self) -> f32 {
        match *self {
            Rate::Day => 0.000_438_6,
            Rate::Night => 0.000_215_5,
            Rate::NightBoost => 0.000_126_5,
            Rate::Purchase => 0.00021,
        }
    }

    #[must_use]
    pub fn cost(&self, power: i32) -> f32 {
        power as f32 * self.value()
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

impl From<Rate> for f32 {
    fn from(rate: Rate) -> Self {
        rate.value()
    }
}
