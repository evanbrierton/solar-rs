use serde::Serialize;
use tabled::Tabled;

use crate::formatting::{euro_to_string, watt_hour_to_string};
use crate::solar_record::SolarRecord;

#[derive(Debug, Tabled, Serialize)]
pub(crate) struct AggregateSolarRecord {
    #[tabled(rename = "Date")]
    key: String,
    #[tabled(rename = "Old Cost", display_with = "euro_to_string")]
    old_cost: f64,
    #[tabled(rename = "New Cost", display_with = "euro_to_string")]
    cost: f64,
    #[tabled(rename = "Savings", display_with = "euro_to_string")]
    savings: f64,
    #[tabled(rename = "Production", display_with = "watt_hour_to_string")]
    production: f64,
    #[tabled(rename = "Consumption", display_with = "watt_hour_to_string")]
    consumption: f64,
    #[tabled(rename = "Purchased", display_with = "watt_hour_to_string")]
    purchased: f64,
    #[tabled(rename = "Feed In", display_with = "watt_hour_to_string")]
    feed_in: f64,
}

macro_rules! getters {
    ($($field:ident),*) => {
        $(
            #[must_use]
            pub fn $field(&self) -> f64 {
                self.$field
            }
        )*
    }
}

impl AggregateSolarRecord {
    #[must_use]
    pub fn new(records: &[SolarRecord], key: &str) -> Self {
        let cost = records.iter().map(SolarRecord::cost).sum::<f64>();
        let old_cost = records.iter().map(SolarRecord::old_cost).sum::<f64>();

        let savings = old_cost - cost;

        macro_rules! construct {
            ($record:expr, $($field:ident),*) => {
                $(let $field = $record.iter().map(|r| r.$field()).sum::<f64>();)*

                return Self {
                    key: key.to_owned(),
                    cost,
                    old_cost,
                    savings,
                    $($field,)*
                }
            }
        }

        construct!(records, production, consumption, purchased, feed_in);
    }

    getters!(
        old_cost,
        cost,
        savings,
        production,
        consumption,
        purchased,
        feed_in
    );
}
