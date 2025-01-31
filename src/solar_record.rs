use chrono::{DateTime, Duration, Utc};

use crate::{rate::Rate, solarman_record::SolarmanRecord};

#[derive(Debug, Clone, Copy)]
pub(crate) struct SolarRecord {
    date_time: DateTime<Utc>,
    duration: Duration,
    production: u32,
    consumption: u32,
    grid: i32,
}

impl SolarRecord {
    #[must_use]
    pub fn new(
        date_time: DateTime<Utc>,
        duration: Duration,
        production: u32,
        consumption: u32,
        grid: i32,
    ) -> Self {
        Self {
            date_time,
            duration,
            production,
            consumption,
            grid,
        }
    }

    #[must_use]
    fn rate(&self) -> Rate {
        self.date_time.into()
    }

    #[must_use]
    pub fn date_time(&self) -> DateTime<Utc> {
        self.date_time
    }

    #[must_use]
    pub fn standing_charge(&self) -> f64 {
        self.rate().standing_charge() * (self.duration.num_minutes() as f64 / 1440_f64)
    }

    #[must_use]
    pub fn old_cost(&self) -> f64 {
        let consumption = i32::try_from(self.consumption).unwrap_or(i32::MAX);
        self.rate().cost(consumption, self.date_time)
            * (self.duration.num_minutes() as f64 / 60_f64)
            + self.standing_charge()
    }

    #[must_use]
    pub fn cost(&self) -> f64 {
        self.rate().cost(-self.grid, self.date_time) * (self.duration.num_minutes() as f64 / 60_f64)
            + self.standing_charge()
    }

    #[must_use]
    pub fn production(&self) -> f64 {
        f64::from(self.production) * (self.duration.num_minutes() as f64 / 60_f64)
    }

    #[must_use]
    pub fn consumption(&self) -> f64 {
        f64::from(self.consumption) * (self.duration.num_minutes() as f64 / 60_f64)
    }

    #[must_use]
    pub fn purchased(&self) -> f64 {
        if self.grid < 0_i32 {
            f64::from(self.grid.abs()) * (self.duration.num_minutes() as f64 / 60_f64)
        } else {
            0_f64
        }
    }

    #[must_use]
    pub fn feed_in(&self) -> f64 {
        if self.grid > 0_i32 {
            f64::from(self.grid) * (self.duration.num_minutes() as f64 / 60_f64)
        } else {
            0_f64
        }
    }

    pub fn from_solarman_record(
        record: &SolarmanRecord,
        start_time: Option<DateTime<Utc>>,
    ) -> Self {
        let duration = match start_time {
            Some(t) => record.time - t,
            None => Duration::minutes(5),
        };

        Self::new(
            record.time,
            duration,
            record.production,
            record.consumption,
            record.grid,
        )
    }
}
