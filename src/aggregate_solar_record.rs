use chrono::NaiveDate;

use crate::solar_record::SolarRecord;

pub struct AggregateSolarRecord {
    date: NaiveDate,
    old: f32,
    new: f32,
    savings: f32,
    production: f32,
    consumption: f32,
}

impl AggregateSolarRecord {
    pub fn new(records: Vec<&SolarRecord>) -> Self {
        let date = records[0].date_time.date_naive();
        let old = records.iter().map(|r| r.old_cost()).sum();
        let new = records.iter().map(|r| r.cost()).sum();
        let savings = records.iter().map(|r| r.savings()).sum();
        let production = records.iter().map(|r| r.production()).sum();
        let consumption = records.iter().map(|r| r.consumption()).sum();

        Self {
            date,
            old,
            new,
            savings,
            production,
            consumption,
        }
    }

    pub fn to_table_row(&self) -> Vec<String> {
        vec![
            self.date.to_string(),
            format!("€{:.2}", self.old),
            format!("€{:.2}", self.new),
            format!("€{:.2}", self.savings),
            format!("{:.2}kWh", self.production / 1000.0),
            format!("{:.2}kWh", self.consumption / 1000.0),
        ]
    }
}
