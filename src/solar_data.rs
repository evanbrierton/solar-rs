use core::fmt::{self, Display, Formatter};
use std::path::Path;

use parsers::{csv, parse_spreadsheets_from_folder};

use chrono::{DateTime, NaiveDate, Utc};
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
