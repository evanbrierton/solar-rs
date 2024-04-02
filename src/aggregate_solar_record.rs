use chrono::Duration;
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
    #[tabled(rename = "Purchased w/o Boost", display_with = "watt_hour_to_string")]
    purchased_without_boost: f64,
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
    const VAT: f64 = 0.09_f64;
    const DAILY_STANDING_CHARGE: f64 = 0.9976_f64;

    #[must_use]
    pub fn new(records: &[SolarRecord], key: &str) -> Self {
        let microgen_credit = records
            .iter()
            .map(SolarRecord::cost)
            .filter(|cost| cost < &0.0_f64)
            .sum::<f64>();

        let duration = records
            .iter()
            .map(SolarRecord::duration)
            .reduce(|acc, duration| acc + duration)
            .unwrap();

        let standing_charge = Self::standing_charge(duration);

        // println!(
        //     "Duration - Standing charge: {} - {}",
        //     duration.num_days(),
        //     standing_charge
        // );

        let raw_cost = records
            .iter()
            .map(SolarRecord::cost)
            .filter(|cost| cost >= &0.0_f64)
            .sum::<f64>();

        // println!("Raw cost: {}", raw_cost);

        let cost = Self::apply_vat(raw_cost + standing_charge) + microgen_credit;

        let old_cost = Self::apply_vat(
            records.iter().map(SolarRecord::old_cost).sum::<f64>() + standing_charge,
        );

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

        construct!(
            records,
            production,
            consumption,
            purchased,
            purchased_without_boost,
            feed_in
        );
    }

    getters!(
        old_cost,
        cost,
        savings,
        production,
        consumption,
        purchased,
        purchased_without_boost,
        feed_in
    );

    fn apply_vat(cost: f64) -> f64 {
        cost * (1.0_f64 + Self::VAT)
    }

    fn standing_charge(duration: Duration) -> f64 {
        Self::DAILY_STANDING_CHARGE * (duration.num_days() as f64)
    }
}
