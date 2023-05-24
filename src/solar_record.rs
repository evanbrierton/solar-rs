use core::num::TryFromIntError;

use chrono::{DateTime, Duration, Utc};

use crate::{rate::Rate, solarman_record::SolarManRecord};

#[derive(Debug, Clone, Copy)]
pub struct SolarRecord {
    pub date_time: DateTime<Utc>,
    duration: Duration,
    production: u16,
    consumption: u32,
    grid: i32,
}

impl SolarRecord {
    #[must_use]
    pub fn new(record: &SolarManRecord, start_time: Option<DateTime<Utc>>) -> Self {
        let duration = match start_time {
            Some(t) => record.time - t,
            None => Duration::minutes(5),
        };

        Self {
            date_time: record.time,
            duration,
            production: record.production,
            consumption: record.consumption,
            grid: record.grid,
        }
    }

    #[must_use]
    pub fn rate(&self) -> Rate {
        if self.grid > 0_i32 {
            return Rate::Purchase;
        }

        self.date_time.into()
    }

    #[must_use]
    pub fn old_rate(&self) -> Rate {
        self.date_time.into()
    }

    #[must_use]
    pub fn savings(&self) -> f32 {
        self.old_cost() - self.cost()
    }

    #[must_use]
    pub fn cost(&self) -> f32 {
        self.rate().cost(-self.grid) * (self.duration.num_minutes() as f32 / 60.0)
    }

    #[must_use]
    pub fn old_cost(&self) -> f32 {
        let consumption = match i32::try_from(self.consumption) {
            Ok(i) => i,
            Err(TryFromIntError { .. }) => {
                eprintln!(
                    "Consumption value too large: {} substituting {} (at record {})",
                    self.consumption,
                    i32::MAX,
                    self.date_time
                );
                i32::MAX
            }
        };

        self.old_rate().cost(consumption) * (self.duration.num_minutes() as f32 / 60.0)
    }

    #[must_use]
    pub fn production(&self) -> f32 {
        f32::from(self.production) * (self.duration.num_minutes() as f32 / 60.0)
    }

    #[must_use]
    pub fn consumption(&self) -> f32 {
        self.consumption as f32 * (self.duration.num_minutes() as f32 / 60.0)
    }

    #[must_use]
    pub fn purchased(&self) -> f32 {
        if self.grid < 0_i32 {
            self.grid.abs() as f32 * (self.duration.num_minutes() as f32 / 60.0)
        } else {
            0.0
        }
    }

    #[must_use]
    pub fn feed_in(&self) -> f32 {
        if self.grid > 0_i32 {
            self.grid as f32 * (self.duration.num_minutes() as f32 / 60.0)
        } else {
            0.0
        }
    }
}
