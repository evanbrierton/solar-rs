use chrono::{DateTime, Timelike, Utc};

pub enum Rate {
    Day,
    Night,
    NightBoost,
    Purchase,
}

impl Rate {
    fn value(&self) -> f32 {
        match self {
            Rate::Day => 0.0004386,
            Rate::Night => 0.0002155,
            Rate::NightBoost => 0.0001265,
            Rate::Purchase => 0.00021,
        }
    }

    pub fn cost(&self, power: i32) -> f32 {
        power as f32 * self.value()
    }
}

impl From<DateTime<Utc>> for Rate {
    fn from(date: DateTime<Utc>) -> Self {
        match date.hour() {
            0..=1 => Rate::Night,
            2..=3 => Rate::NightBoost,
            4..=7 => Rate::Day,
            8..=23 => Rate::Night,
            _ => unreachable!(),
        }
    }
}

impl From<Rate> for f32 {
    fn from(rate: Rate) -> Self {
        rate.value()
    }
}
