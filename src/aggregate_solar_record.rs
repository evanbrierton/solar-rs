use chrono::{DateTime, Utc};
use tabled::Tabled;

use crate::formatting::{euro_to_string, kwh_to_string};
use crate::rate::Rate;
use crate::solar_record::SolarRecord;

#[derive(Debug, Tabled)]
pub struct AggregateSolarRecord {
    #[tabled(rename = "Date")]
    key: String,
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
    #[tabled(rename = "Purchased w/o Boost", display_with = "kwh_to_string")]
    purchased_without_boost: f64,
    #[tabled(rename = "Feed In", display_with = "kwh_to_string")]
    feed_in: f64,
}

impl AggregateSolarRecord {
    #[must_use]
    pub fn new(records: &[SolarRecord], key: &str) -> Self {
        let sum = |f: fn(&SolarRecord) -> f64| records.iter().map(f).sum::<f64>();

        let old_cost = sum(SolarRecord::old_cost);
        let new_cost = sum(SolarRecord::cost);
        let savings = sum(SolarRecord::savings);
        let production = sum(SolarRecord::production);
        let consumption = sum(SolarRecord::consumption);
        let purchased = sum(SolarRecord::purchased);
        let feed_in = sum(SolarRecord::feed_in);

        let purchased_without_boost = records
            .iter()
            .filter(|r| !(r.rate() == Rate::NightBoost))
            .map(SolarRecord::purchased)
            .sum();

        Self {
            key: key.to_owned(),
            old_cost,
            new_cost,
            savings,
            production,
            consumption,
            purchased,
            purchased_without_boost,
            feed_in,
        }
    }

    #[must_use]
    pub fn to_table_row(&self) -> Vec<String> {
        vec![
            self.key.clone(),
            euro_to_string(&self.old_cost),
            euro_to_string(&self.new_cost),
            euro_to_string(&self.savings),
            kwh_to_string(&self.production),
            kwh_to_string(&self.consumption),
            kwh_to_string(&self.purchased),
            kwh_to_string(&self.feed_in),
        ]
    }
}

pub enum Period {
    Minute,
    Hour,
    Day,
    Month,
    Year,
}

impl Period {
    #[must_use]
    pub fn key(&self, date: &DateTime<Utc>) -> String {
        match self {
            Self::Minute => format!("{}", date.format("%Y-%m-%d %H:%M")),
            Self::Hour => format!("{}", date.format("%Y-%m-%d %H")),
            Self::Day => format!("{}", date.format("%Y-%m-%d")),
            Self::Month => format!("{}", date.format("%Y-%m")),
            Self::Year => format!("{}", date.format("%Y")),
        }
    }
}
