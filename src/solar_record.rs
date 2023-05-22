use chrono::{DateTime, Duration, Utc};

use crate::{rate::Rate, solarman_record::SolarManRecord};

pub struct SolarRecord {
    pub date_time: DateTime<Utc>,
    duration: Duration,
    production: u32,
    consumption: u32,
    grid: i32,
}

impl SolarRecord {
    pub fn new(record: &SolarManRecord, start_time: Option<DateTime<Utc>>) -> Self {
        let duration = match start_time {
            Some(start_time) => record.time - start_time,
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

    pub fn savings(&self) -> f32 {
        self.old_cost() - self.cost()
    }

    pub fn cost(&self) -> f32 {
        self.rate().cost(-self.grid) * (self.duration.num_minutes() as f32 / 60.0)
    }

    pub fn old_cost(&self) -> f32 {
        self.old_rate().cost(self.consumption as i32) * (self.duration.num_minutes() as f32 / 60.0)
    }

    pub fn production(&self) -> f32 {
        self.production as f32 * (self.duration.num_minutes() as f32 / 60.0)
    }

    pub fn consumption(&self) -> f32 {
        self.consumption as f32 * (self.duration.num_minutes() as f32 / 60.0)
    }

    pub fn purchased(&self) -> f32 {
        if self.grid < 0 {
            self.grid.abs() as f32 * (self.duration.num_minutes() as f32 / 60.0)
        } else {
            0.0
        }
    }

    pub fn feed_in(&self) -> f32 {
        if self.grid > 0 {
            self.grid as f32 * (self.duration.num_minutes() as f32 / 60.0)
        } else {
            0.0
        }
    }
}
