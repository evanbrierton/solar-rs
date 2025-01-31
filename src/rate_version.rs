use std::cmp::Ordering;

use chrono::{DateTime, TimeZone, Timelike, Utc};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Eq)]
pub(crate) enum Rate {
    ElectricIrelandV0,
    ElectricIrelandV1,
    ElectricIrelandV2,
    EnergiaV0,
}

impl Rate {
    fn value(&self) -> DateTime<Utc> {
        match *self {
            Rate::ElectricIrelandV0 => Utc.with_ymd_and_hms(1, 1, 1, 0, 0, 0).unwrap(),
            Rate::ElectricIrelandV1 => Utc.with_ymd_and_hms(2024, 3, 1, 0, 0, 0).unwrap(),
            Rate::ElectricIrelandV2 => Utc.with_ymd_and_hms(2024, 11, 1, 0, 0, 0).unwrap(),
            Rate::EnergiaV0 => Utc.with_ymd_and_hms(2025, 1, 15, 0, 0, 0).unwrap(),
        }
    }

    pub fn standing_charge(&self) -> f64 {
        match self {
            Rate::ElectricIrelandV0 => 0.9976_f64,
            Rate::ElectricIrelandV1 => 0.9976_f64,
            Rate::ElectricIrelandV2 => 0.8259_f64,
            Rate::EnergiaV0 => 0.6482739726_f64,
        }
    }

    pub fn feed_in(&self) -> f64 {
        (match self {
            Rate::ElectricIrelandV0 => 0.21_f64,
            Rate::ElectricIrelandV1 => 0.21_f64,
            Rate::ElectricIrelandV2 => 0.195_f64,
            Rate::EnergiaV0 => 0.20_f64,
        } / 1000_f64)
    }

    fn evaluate(&self, date: DateTime<Utc>) -> f64 {
        (match self {
            Rate::ElectricIrelandV0 => match date.hour() {
                23 | 0..=1 | 4..=7 => 0.2092_f64,
                2..=3 => 0.1228_f64,
                8..=22 => 0.4008_f64,
                _ => unreachable!("Invalid hour"),
            },
            Rate::ElectricIrelandV1 => match date.hour() {
                23 | 0..=1 | 4..=7 => 0.1783_f64,
                2..=3 => 0.1047_f64,
                8..=22 => 0.3615_f64,
                _ => unreachable!("Invalid hour"),
            },
            Rate::ElectricIrelandV2 => match date.hour() {
                23 | 0..=1 | 4..=7 => 0.1783_f64,
                2..=3 => 0.1047_f64,
                8..=22 => 0.3615_f64,
                _ => unreachable!("Invalid hour"),
            },
            Rate::EnergiaV0 => match date.hour() {
                23 | 0..=7 => 0.1349_f64,
                8..=16 | 19..=22 => 0.2521_f64,
                17..=18 => 0.2642_f64,
                _ => unreachable!("Invalid hour"),
            },
        } / 1000_f64)
    }

    pub fn cost(&self, consumption: i32, date: DateTime<Utc>) -> f64 {
        if consumption < 0 {
            return self.feed_in() * consumption as f64 * self.evaluate(date);
        }

        consumption as f64 * self.evaluate(date)
    }
}

impl Ord for Rate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Rate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<DateTime<Utc>> for Rate {
    fn from(date: DateTime<Utc>) -> Self {
        return Rate::iter()
            .filter(|version| date > version.value())
            .max()
            .unwrap_or_default();
    }
}

impl Default for Rate {
    fn default() -> Self {
        Rate::iter().last().unwrap_or(Rate::ElectricIrelandV0)
    }
}
