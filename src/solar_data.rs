use crate::solar_record::SolarRecord;

pub struct SolarData {
    records: Vec<SolarRecord>,
}

impl SolarData {
    pub fn new(records: Vec<SolarRecord>) -> Self {
        Self { records }
    }

    pub fn records(&self) -> &[SolarRecord] {
        &self.records
    }

    pub fn records_mut(&mut self) -> &mut [SolarRecord] {
        &mut self.records
    }

    pub fn total_savings(&self) -> f32 {
        self.records
            .iter()
            .map(|record| record.savings(&record.duration()))
            .sum()
    }

    pub fn total_production(&self) -> f32 {
        self.records
            .iter()
            .map(|record| record.production(&record.duration()))
            .sum()
    }

    pub fn total_consumption(&self) -> f32 {
        self.records
            .iter()
            .map(|record| record.consumption(&record.duration()))
            .sum()
    }
}
