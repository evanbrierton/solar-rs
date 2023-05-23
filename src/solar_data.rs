use std::{
    fmt::{self, Display, Formatter},
    path::Path,
};

use chrono::{DateTime, Utc};
use itertools::Itertools;
use tabled::{
    builder::Builder,
    settings::{Concat, Style},
    Table,
};

use crate::{
    aggregate_solar_record::AggregateSolarRecord,
    formatting::{euro_to_string, kwh_to_string},
    parsers::parse_spreadsheets_from_folder,
    solar_record::SolarRecord,
    solarman_record::SolarManRecord,
};

const SETUP_COST: f32 = 11_000.0;

pub struct SolarData {
    pub setup_cost: f32,
    records: Vec<SolarRecord>,
}

impl SolarData {
    #[must_use]
    pub fn new(setup_cost: f32, records: Vec<SolarRecord>) -> Self {
        Self {
            setup_cost,
            records,
        }
    }

    pub fn append(&mut self, mut records: Vec<SolarRecord>) {
        self.records.append(&mut records);
    }

    #[must_use]
    pub fn aggregate(&self) -> Vec<AggregateSolarRecord> {
        self.records
            .iter()
            .copied()
            .group_by(|r| r.date_time.date_naive())
            .into_iter()
            .map(|(_, records)| AggregateSolarRecord::new(&records.collect::<Vec<_>>()))
            .collect()
    }

    #[must_use]
    pub fn cost(&self) -> f32 {
        self.records.iter().map(SolarRecord::cost).sum()
    }

    #[must_use]
    pub fn old_cost(&self) -> f32 {
        self.records.iter().map(SolarRecord::old_cost).sum()
    }

    #[must_use]
    pub fn savings(&self) -> f32 {
        self.records.iter().map(SolarRecord::savings).sum()
    }

    #[must_use]
    pub fn production(&self) -> f32 {
        self.records.iter().map(SolarRecord::production).sum()
    }

    #[must_use]
    pub fn consumption(&self) -> f32 {
        self.records.iter().map(SolarRecord::consumption).sum()
    }

    #[must_use]
    pub fn purchased(&self) -> f32 {
        self.records.iter().map(SolarRecord::purchased).sum()
    }

    #[must_use]
    pub fn feed_in(&self) -> f32 {
        self.records.iter().map(SolarRecord::feed_in).sum()
    }

    #[must_use]
    pub fn mean_savings(&self) -> f32 {
        let length = self.aggregate().len() as f32;

        self.savings() / self.aggregate().len() as f32
    }

    #[must_use]
    pub fn remaining_setup_cost(&self) -> f32 {
        self.setup_cost - self.savings()
    }

    #[must_use]
    pub fn remaining_days(&self) -> f32 {
        self.remaining_setup_cost() / self.mean_savings()
    }

    #[must_use]
    pub fn payoff_date(&self) -> DateTime<Utc> {
        Utc::now() + chrono::Duration::days(self.remaining_days() as i64)
    }

    /// # Errors
    /// # Panics
    pub fn from_folder<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let raw_records = parse_spreadsheets_from_folder::<SolarManRecord, _>(path)?;
        let sorted_raw_records = raw_records
            .iter()
            .sorted_by_key(|solarman_record| solarman_record.time)
            .collect::<Vec<_>>();

        let records = sorted_raw_records
            .iter()
            .enumerate()
            .map(|(i, record)| {
                let start_time = match i {
                    0 => None,
                    _ => Some(sorted_raw_records[i - 1].time),
                };

                SolarRecord::new(record, start_time)
            })
            .collect::<Vec<_>>();

        Ok(Self::new(SETUP_COST, records))
    }
}

impl Display for SolarData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut table = Table::new(self.aggregate());
        table.with(Style::rounded());

        let mut builder = Builder::from_iter([[
            "Total".to_string(),
            euro_to_string(&self.old_cost()),
            euro_to_string(&self.cost()),
            euro_to_string(&self.savings()),
            kwh_to_string(&self.production()),
            kwh_to_string(&self.consumption()),
            kwh_to_string(&self.purchased()),
            kwh_to_string(&self.feed_in()),
        ]]);

        builder.remove_header();
        let total = builder.build();

        table.with(Concat::vertical(total));

        let output = format!(
            "{}\n{}\n{}\n{}\n",
            table,
            self.mean_savings(),
            self.remaining_setup_cost(),
            self.payoff_date().to_rfc2822()
        );

        write!(f, "{}", output)
    }
}
