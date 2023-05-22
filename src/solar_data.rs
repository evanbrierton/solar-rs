use std::{
    fmt::{self, Display, Formatter},
    path::Path,
};

use chrono::{DateTime, Utc};
use itertools::Itertools;
use tabled::{builder::Builder, settings::Style};

use crate::{
    aggregate_solar_record::AggregateSolarRecord, parse::read_from_file, solar_record::SolarRecord,
    solarman_record::SolarManRecord,
};

const SETUP_COST: f32 = 11_000.0;

pub struct SolarData {
    pub setup_cost: f32,
    records: Vec<SolarRecord>,
}

impl SolarData {
    pub fn new(setup_cost: f32, records: Vec<SolarRecord>) -> Self {
        Self {
            setup_cost,
            records,
        }
    }

    pub fn append(&mut self, mut records: Vec<SolarRecord>) {
        self.records.append(&mut records);
    }

    pub fn aggregate(&self) -> Vec<AggregateSolarRecord> {
        self.records
            .iter()
            .group_by(|r| r.date_time.date_naive())
            .into_iter()
            .map(|(_, records)| AggregateSolarRecord::new(records.collect()))
            .collect()
    }

    pub fn cost(&self) -> f32 {
        self.records.iter().map(|r| r.cost()).sum()
    }

    pub fn old_cost(&self) -> f32 {
        self.records.iter().map(|r| r.old_cost()).sum()
    }

    pub fn savings(&self) -> f32 {
        self.records.iter().map(|r| r.savings()).sum()
    }

    pub fn production(&self) -> f32 {
        self.records.iter().map(|r| r.production()).sum()
    }

    pub fn consumption(&self) -> f32 {
        self.records.iter().map(|r| r.consumption()).sum()
    }

    pub fn purchased(&self) -> f32 {
        self.records.iter().map(|r| r.purchased()).sum()
    }

    pub fn feed_in(&self) -> f32 {
        self.records.iter().map(|r| r.feed_in()).sum()
    }

    pub fn mean_savings(&self) -> f32 {
        self.savings() / self.records.len() as f32
    }

    pub fn remaining_setup_cost(&self) -> f32 {
        self.setup_cost - self.savings()
    }

    pub fn remaining_days(&self) -> f32 {
        self.remaining_setup_cost() / self.mean_savings()
    }

    pub fn payoff_date(&self) -> DateTime<Utc> {
        Utc::now() + chrono::Duration::days(self.remaining_days() as i64)
    }

    pub fn from_folder<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let directory_elements = std::fs::read_dir(path)?.collect::<Result<Vec<_>, _>>()?;

        let csv_files = directory_elements
            .into_iter()
            .filter(|entry| {
                let is_file = entry.file_type().map(|ft| ft.is_file()).unwrap_or(false);
                let has_csv_extension = entry
                    .path()
                    .extension()
                    .map(|ext| ext == "csv")
                    .unwrap_or(false);

                is_file && has_csv_extension
            })
            .collect::<Vec<_>>();

        let sorted_raw_records = csv_files
            .into_iter()
            .map(|file_entry| read_from_file::<SolarManRecord, _>(file_entry.path()))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .sorted_by_key(|solarman_record| solarman_record.time)
            .collect::<Vec<_>>();

        let records = sorted_raw_records
            .iter()
            .enumerate()
            .map(|(index, record)| {
                let start_time = if index == 0 {
                    None
                } else {
                    Some(sorted_raw_records[index - 1].time)
                };

                SolarRecord::new(record, start_time)
            })
            .collect::<Vec<_>>();

        Ok(Self::new(SETUP_COST, records))
    }
}

impl Display for SolarData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let headers = vec![
            "Date",
            "Old Cost",
            "New Cost",
            "Savings",
            "Production",
            "Consumption",
            "Purchased",
            "Feed In",
        ];

        let mut builder = Builder::default();
        builder.set_header(headers);

        for record in self.aggregate() {
            builder.push_record(record.to_table_row());
        }

        let total = vec![
            "Total".to_string(),
            format!("€{:.2}", self.old_cost()),
            format!("€{:.2}", self.cost()),
            format!("€{:.2}", self.savings()),
            format!("{:.2}kWh", self.production() / 1000.0),
            format!("{:.2}kWh", self.consumption() / 1000.0),
            format!("{:.2}kWh", self.purchased() / 1000.0),
            format!("{:.2}kWh", self.feed_in() / 1000.0),
        ];

        builder.push_record(total);

        let table = builder.build().with(Style::rounded()).to_string();

        write!(f, "{}", table)
    }
}
