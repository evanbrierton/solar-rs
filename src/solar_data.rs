use core::fmt::{self, Display, Formatter};
use std::path::Path;

use parsers::{csv, parse_spreadsheets_from_folder};

use chrono::{DateTime, NaiveDate, Timelike, Utc};
use itertools::Itertools;
use tabled::{
    builder::Builder,
    settings::{Concat, Style},
    Table,
};

use crate::{
    aggregate_solar_record::AggregateSolarRecord,
    formatting::{euro_to_string, watt_hour_to_string},
    period::Period,
    solar_record::SolarRecord,
    solarman_record::SolarmanRecord,
};

#[derive(Debug)]
pub struct SolarData {
    setup_cost: f64,
    records: Vec<SolarRecord>,
    aggregation_period: Period,
    limit: usize,
}

macro_rules! metrics {
    ($($metric:ident),*) => {
        $(pub(crate) fn $metric(&self) -> f64 {
            self.aggregate(self.aggregation_period)
                .iter()
                .map(AggregateSolarRecord::$metric)
                .sum::<f64>()
        })*
    }
}

impl SolarData {
    #[must_use]
    pub(crate) fn new(
        setup_cost: f64,
        records: Vec<SolarRecord>,
        aggregation_period: Period,
        limit: usize,
    ) -> Self {
        Self {
            setup_cost,
            records,
            aggregation_period,
            limit,
        }
    }

    #[must_use]
    pub(crate) fn aggregate(&self, period: Period) -> Vec<AggregateSolarRecord> {
        let groups = self.records.iter().group_by(|r| period.key(&r.date_time()));

        let labelled_groups = groups.into_iter().map(|(date, records)| {
            AggregateSolarRecord::new(&records.copied().collect::<Vec<_>>(), &date)
        });

        labelled_groups.collect::<Vec<_>>()
    }

    metrics! {
        old_cost,
        cost,
        savings,
        production,
        consumption,
        purchased,
        purchased_without_boost,
        feed_in
    }

    #[must_use]
    pub(crate) fn mean_savings(&self, period: Period) -> f64 {
        self.savings() / self.aggregate(period).len() as f64
    }

    #[must_use]
    pub(crate) fn remaining_setup_cost(&self) -> f64 {
        self.setup_cost - self.savings()
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn remaining_days(&self) -> i64 {
        (self.remaining_setup_cost() / self.mean_savings(Period::Day)).round() as i64
    }

    #[must_use]
    pub(crate) fn payoff_date(&self) -> NaiveDate {
        (Utc::now() + chrono::Duration::days(self.remaining_days())).date_naive()
    }

    #[inline]
    pub fn write<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        csv::write(path, &self.aggregate(self.aggregation_period))
    }

    /// # Errors
    /// # Panics
    #[inline]
    pub fn from_folder<P: AsRef<Path>>(
        path: P,
        aggregation_period: Period,
        setup_cost: f64,
        limit: usize,
    ) -> anyhow::Result<Self> {
        let raw_records = parse_spreadsheets_from_folder::<SolarmanRecord, _>(path)?;
        let sorted_raw_records = raw_records
            .iter()
            .sorted_by_key(|solarman_record| solarman_record.time)
            .collect::<Vec<_>>();

        let mut start_time: Option<DateTime<Utc>> = None;

        let records = sorted_raw_records
            .iter()
            .map(|raw_record| {
                let record = SolarRecord::from_solarman_record(raw_record, start_time);
                start_time = Some(record.date_time());
                record
            })
            .collect::<Vec<_>>();

        Ok(Self::new(setup_cost, records, aggregation_period, limit))
    }

    #[inline]
    #[must_use]
    pub fn simulate(
        &self,
        battery_capacity: f64,
        battery_charge_rate: u32,
    ) -> (Vec<i32>, Vec<i32>) {
        let (grid, battery, _) = self
            .records
            .iter()
            .fold((vec![], vec![], 0_f64), |(mut grid, mut battery, isoc), r| {
                let duration = r.duration().num_seconds();
                let max_charge = (battery_charge_rate as i64 * duration) as f64 / 3600_f64;

                let mut delta = r.production() - r.consumption();
                let initial_delta = delta;

                let mut soc = match delta {
                    _ if delta > 0_f64 => {
                        if isoc < battery_capacity {
                            let new_soc =
                                (isoc + delta.min(max_charge)).min(battery_capacity);
                            delta -= new_soc - isoc;
                            new_soc
                        } else {
                            isoc
                        }
                    }
                    _ if delta < 0_f64 => {
                        if isoc > 0.2_f64 * battery_capacity {
                            let new_soc = (isoc + delta.min(max_charge)).max(0_f64);
                            delta += isoc - new_soc;
                            new_soc
                        } else {
                            isoc
                        }
                    }
                    _ => isoc,
                };

                soc = if r.date_time().hour() >= 2 && r.date_time().hour() < 4 {
                    if soc < battery_capacity {
                        let new_soc = (soc + max_charge).min(battery_capacity);
                        delta -= new_soc - soc;
                        new_soc
                    } else {
                        soc
                    }
                } else {
                    soc
                };

                grid.push(delta);
                battery.push(soc - isoc);

                println!(
                    "{} - Production: {}, Consumption: {}, Grid: {}, P-Grid: {}, Battery: {}, P-Battery: {} -- Initial Delta: {}",
                    r.date_time(),
                    r.production().round(),
                    r.consumption().round(),
                    r.grid().round(),
                    delta.round(),
                    r.battery().round(),
                    (soc - isoc).round(),
                    initial_delta.round(),
                );

                (grid, battery, soc)
            });

        (
            grid.iter().map(|x| x.round() as i32).collect(),
            battery.iter().map(|x| x.round() as i32).collect(),
        )
    }

    #[inline]
    #[must_use]
    pub fn with_additional_battery(
        &self,
        battery_capacity: f64,
        battery_charge_rate: u32,
        battery_cost: f64,
    ) -> Self {
        let (grid, battery) = self.simulate(battery_capacity, battery_charge_rate);

        let records = self
            .records
            .iter()
            .zip(grid.iter().zip(battery.iter()))
            .map(|(r, (grid, battery))| r.with_grid_and_battery(*grid, *battery))
            .collect::<Vec<_>>();

        Self::new(
            self.setup_cost + battery_cost,
            records,
            self.aggregation_period,
            self.limit,
        )
    }

    #[inline]
    pub fn test_simulate(&self, battery_capacity: f64, battery_charge_rate: u32) {
        let mut delta_sum = 0;

        for (r, grid) in self
            .records
            .iter()
            .zip(self.simulate(battery_capacity, battery_charge_rate).0)
        {
            delta_sum += r.grid() as i32 - grid;
        }

        let mean_delta = delta_sum as f64 / self.records.len() as f64;

        println!("Mean delta: {}", mean_delta);
    }
}

impl Display for SolarData {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let period = self.aggregation_period;

        let aggregate_records = self.aggregate(period);

        let mut total_builder = Builder::from_iter([[
            "Total".to_owned(),
            euro_to_string(&self.old_cost()),
            euro_to_string(&self.cost()),
            euro_to_string(&self.savings()),
            watt_hour_to_string(&self.production()),
            watt_hour_to_string(&self.consumption()),
            watt_hour_to_string(&self.purchased()),
            watt_hour_to_string(&self.purchased_without_boost()),
            watt_hour_to_string(&self.feed_in()),
        ]]);

        total_builder.remove_header();
        let total = total_builder.build();

        let mut mean_builder = Builder::from_iter([[
            "Mean".to_owned(),
            euro_to_string(&(self.old_cost() / self.aggregate(period).len() as f64)),
            euro_to_string(&(self.cost() / self.aggregate(period).len() as f64)),
            euro_to_string(&(self.savings() / self.aggregate(period).len() as f64)),
            watt_hour_to_string(&(self.production() / self.aggregate(period).len() as f64)),
            watt_hour_to_string(&(self.consumption() / self.aggregate(period).len() as f64)),
            watt_hour_to_string(&(self.purchased() / self.aggregate(period).len() as f64)),
            watt_hour_to_string(
                &(self.purchased_without_boost() / self.aggregate(period).len() as f64),
            ),
            watt_hour_to_string(&(self.feed_in() / self.aggregate(period).len() as f64)),
        ]]);

        mean_builder.remove_header();
        let mean = mean_builder.build();

        let mut table = Table::new(aggregate_records.iter().rev().take(self.limit).rev());
        table.with(Style::rounded());

        table.with(Concat::vertical(mean));
        table.with(Concat::vertical(total));

        let output = format!(
            "{table}\nRemaining Balance: €{:.2}\nExpected Payoff Date: {:.2}\n",
            self.remaining_setup_cost(),
            self.payoff_date()
        );

        write!(f, "{output}")
    }
}
