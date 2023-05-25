use core::num::TryFromIntError;

use chrono::{DateTime, Duration, Utc};

use crate::{rate::Rate, solarman_record::SolarmanRecord};

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
    pub fn new(record: &SolarmanRecord, start_time: Option<DateTime<Utc>>) -> Self {
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
            return Rate::FeedIn;
        }

        self.date_time.into()
    }

    #[must_use]
    pub fn old_rate(&self) -> Rate {
        self.date_time.into()
    }

    #[must_use]
    pub fn savings(&self) -> f64 {
        self.old_cost() - self.cost()
    }

    #[must_use]
    pub fn cost(&self) -> f64 {
        self.rate().cost(-self.grid) * (self.duration.num_minutes() as f64 / 60_f64)
    }

    #[must_use]
    pub fn old_cost(&self) -> f64 {
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

        self.old_rate().cost(consumption) * (self.duration.num_minutes() as f64 / 60_f64)
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{anyhow, ensure};
    use chrono::{Duration, Utc};

    fn fuzzy_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < f64::EPSILON
    }

    #[test]
    fn test_new() -> anyhow::Result<()> {
        let record = SolarmanRecord {
            time: Utc::now(),
            production: 100,
            consumption: 50,
            grid: 0,
            battery: 0,
            soc: 0,
        };

        let start_time = Some(Utc::now() - Duration::minutes(5));
        let solar_record = SolarRecord::new(&record, start_time);

        ensure!(
            solar_record.production == 100,
            anyhow!("production mismatch")
        );

        ensure!(
            solar_record.consumption == 50,
            anyhow!("consumption mismatch")
        );

        ensure!(solar_record.grid == 0_i32, anyhow!("grid mismatch"));

        ensure!(
            solar_record.duration == Duration::minutes(0),
            anyhow!("duration mismatch")
        );

        Ok(())
    }

    #[test]
    fn test_rate() -> anyhow::Result<()> {
        let record = SolarRecord {
            date_time: Utc::now(),
            duration: Duration::minutes(60),
            production: 100,
            consumption: 50,
            grid: -25,
        };

        ensure!(record.rate() == Rate::FeedIn, anyhow!("rate mismatch"));

        Ok(())
    }

    #[test]
    fn test_old_rate() -> anyhow::Result<()> {
        let record = SolarRecord {
            date_time: Utc::now(),
            duration: Duration::minutes(60),
            production: 100,
            consumption: 50,
            grid: -25,
        };

        ensure!(
            record.old_rate() == Rate::FeedIn,
            anyhow!("old rate mismatch")
        );

        Ok(())
    }

    #[test]
    fn test_savings() -> anyhow::Result<()> {
        let record = SolarRecord {
            date_time: Utc::now(),
            duration: Duration::minutes(60),
            production: 100,
            consumption: 50,
            grid: -25,
        };

        ensure!(record.savings() == 0_f64, anyhow!("savings mismatch"));

        Ok(())
    }

    #[test]
    fn test_cost() -> anyhow::Result<()> {
        let record = SolarRecord {
            date_time: Utc::now(),
            duration: Duration::minutes(60),
            production: 100,
            consumption: 50,
            grid: -25,
        };

        ensure!(record.cost() == 0_f64, anyhow!("cost mismatch"));
        Ok(())
    }

    #[test]
    fn test_old_cost() -> anyhow::Result<()> {
        let record = SolarRecord {
            date_time: Utc::now(),
            duration: Duration::minutes(60),
            production: 100,
            consumption: 50,
            grid: -25,
        };

        ensure!(record.old_cost() == 0_f64, anyhow!("old cost mismatch"));

        Ok(())
    }

    #[test]
    fn test_production() -> anyhow::Result<()> {
        let record = SolarRecord {
            date_time: Utc::now(),
            duration: Duration::minutes(60),
            production: 100,
            consumption: 50,
            grid: -25,
        };

        ensure!(
            fuzzy_eq(record.production(), 100.0),
            anyhow!("production mismatch")
        );

        Ok(())
    }

    #[test]
    fn test_consumption() -> anyhow::Result<()> {
        let record = SolarRecord {
            date_time: Utc::now(),
            duration: Duration::minutes(60),
            production: 100,
            consumption: 50,
            grid: -25,
        };

        ensure!(
            fuzzy_eq(record.consumption(), 50.0),
            anyhow!("consumption mismatch")
        );

        Ok(())
    }

    #[test]
    fn test_purchased() -> anyhow::Result<()> {
        let record = SolarRecord {
            date_time: Utc::now(),
            duration: Duration::minutes(60),
            production: 100,
            consumption: 50,
            grid: -25,
        };

        ensure!(
            fuzzy_eq(record.purchased(), 25.0),
            anyhow!("purchased mismatch")
        );

        Ok(())
    }

    #[test]
    fn test_feed_in() -> anyhow::Result<()> {
        let record = SolarRecord {
            date_time: Utc::now(),
            duration: Duration::minutes(60),
            production: 100,
            consumption: 50,
            grid: 25,
        };

        ensure!(
            fuzzy_eq(record.feed_in(), 25.0),
            anyhow!("feed in mismatch")
        );

        Ok(())
    }
}
