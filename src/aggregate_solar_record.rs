use chrono::NaiveDate;
use tabled::Tabled;

use crate::formatting::{euro_to_string, kwh_to_string};
use crate::solar_record::SolarRecord;

#[derive(Tabled)]
pub struct AggregateSolarRecord {
    #[tabled(rename = "Date", display_with = "NaiveDate::to_string")]
    date: NaiveDate,
    #[tabled(rename = "Old Cost", display_with = "euro_to_string")]
    old_cost: f64,
    #[tabled(rename = "New Cost", display_with = "euro_to_string")]
    new_cost: f64,
    #[tabled(rename = "Savings", display_with = "euro_to_string")]
    savings: f64,
    #[tabled(rename = "Production", display_with = "kwh_to_string")]
    production: f64,
    #[tabled(rename = "Consumption", display_with = "kwh_to_string")]
    consumption: f64,
    #[tabled(rename = "Purchased", display_with = "kwh_to_string")]
    purchased: f64,
    #[tabled(rename = "Feed In", display_with = "kwh_to_string")]
    feed_in: f64,
}

impl AggregateSolarRecord {
    #[must_use]
    pub fn new(records: &[SolarRecord], date: NaiveDate) -> Self {
        let sum = |f: fn(&SolarRecord) -> f64| records.iter().map(f).sum::<f64>();

        let old_cost = sum(SolarRecord::old_cost);
        let new_cost = sum(SolarRecord::cost);
        let savings = sum(SolarRecord::savings);
        let production = sum(SolarRecord::production);
        let consumption = sum(SolarRecord::consumption);
        let purchased = sum(SolarRecord::purchased);
        let feed_in = sum(SolarRecord::feed_in);

        Self {
            date,
            old_cost,
            new_cost,
            savings,
            production,
            consumption,
            purchased,
            feed_in,
        }
    }

    #[must_use]
    pub fn to_table_row(&self) -> Vec<String> {
        vec![
            self.date.to_string(),
            format!("€{:.2}", self.old_cost),
            format!("€{:.2}", self.new_cost),
            format!("€{:.2}", self.savings),
            format!("{:.2}kWh", self.production / 1000_f64),
            format!("{:.2}kWh", self.consumption / 1000_f64),
            format!("{:.2}kWh", self.purchased / 1000_f64),
            format!("{:.2}kWh", self.feed_in / 1000_f64),
        ]
    }
}
