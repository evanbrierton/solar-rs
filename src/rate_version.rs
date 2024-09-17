use std::cmp::Ordering;

use chrono::{DateTime, TimeZone, Utc};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Eq)]
pub(crate) enum RateVersion {
    V0,
    V1,
}

impl RateVersion {
    fn value(&self) -> DateTime<Utc> {
        match *self {
            RateVersion::V0 => Utc.with_ymd_and_hms(1, 1, 1, 0, 0, 0).unwrap(),
            RateVersion::V1 => Utc.with_ymd_and_hms(2024, 3, 1, 0, 0, 0).unwrap(),
        }
    }
}

impl Ord for RateVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for RateVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<DateTime<Utc>> for RateVersion {
    fn from(date: DateTime<Utc>) -> Self {
        return RateVersion::iter()
            .filter(|version| date > version.value())
            .max()
            .unwrap_or_default();
    }
}

impl Default for RateVersion {
    fn default() -> Self {
        RateVersion::iter().last().unwrap_or(RateVersion::V0)
    }
}
