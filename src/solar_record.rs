use chrono::{DateTime, Duration, NaiveDate, NaiveTime, Utc};

use crate::{rate::Rate, solarman_record::SolarManRecord};

pub struct SolarRecord {
    date: NaiveDate,
    time: NaiveTime,
    pub date_time: DateTime<Utc>,
    duration: Duration,
    production: u32,
    consumption: u32,
    grid: i32,
    battery: i32,
    soc: u8,
}

impl SolarRecord {
    pub fn new(record: &SolarManRecord, start_time: Option<DateTime<Utc>>) -> Self {
        let duration = match start_time {
            Some(start_time) => record.time - start_time,
            None => Duration::minutes(5),
        };

        Self {
            time: record.time.time(),
            date: record.time.date_naive(),
            date_time: record.time,
            duration,
            production: record.production,
            consumption: record.consumption,
            grid: record.grid,
            battery: record.battery,
            soc: record.soc,
        }
    }
}

impl SolarRecord {
    pub fn rate(&self) -> Rate {
        if self.grid > 0 {
            return Rate::Purchase;
        }

        self.date_time.into()
    }

    pub fn old_rate(&self) -> Rate {
        self.date_time.into()
    }

    pub fn cost(&self) -> f32 {
        self.rate().cost(-self.grid) * (self.duration.num_minutes() as f32 / 60.0)
    }

    pub fn old_cost(&self) -> f32 {
        self.old_rate().cost(self.consumption as i32) * (self.duration.num_minutes() as f32 / 60.0)
    }

    pub fn savings(&self) -> f32 {
        self.old_cost() - self.cost()
    }

    pub fn production(&self) -> f32 {
        self.production as f32 * (self.duration.num_minutes() as f32 / 60.0)
    }

    pub fn consumption(&self) -> f32 {
        self.consumption as f32 * (self.duration.num_minutes() as f32 / 60.0)
    }
}
