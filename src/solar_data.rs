use std::{
    fmt::{self, Display, Formatter},
    fs::ReadDir,
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
        self.old_cost() - self.cost()
    }

    pub fn production(&self) -> f32 {
        self.records.iter().map(|r| r.production()).sum()
    }

    pub fn consumption(&self) -> f32 {
        self.records.iter().map(|r| r.consumption()).sum()
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
}

impl From<ReadDir> for SolarData {
    fn from(dir: ReadDir) -> Self {
        let start_time = None;
        let mut records = Vec::new();

        for entry in dir.sorted_by_key(|dir| dir.as_ref().unwrap().path()) {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();

            if file_name.ends_with(".csv") {
                let raw_record = read_from_file::<SolarManRecord>(path.to_str().unwrap()).unwrap();
                let mut record = raw_record
                    .iter()
                    .map(|r| SolarRecord::new(r, start_time))
                    .collect::<Vec<_>>();

                records.append(&mut record);
            }
        }

        Self::new(SETUP_COST, records)
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
        ];

        let mut builder = Builder::default();
        builder.set_header(headers);

        for record in self.aggregate() {
            builder.push_record(record.to_table_row());
        }

        let table = builder.build().with(Style::rounded()).to_string();

        write!(f, "{}", table)
    }
}
