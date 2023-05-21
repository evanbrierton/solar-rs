use std::error::Error;

use serde::{Deserialize, Serialize};

pub fn read_from_file<T: for<'de> Deserialize<'de>>(path: &str) -> Result<Vec<T>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records = Vec::new();

    for result in rdr.deserialize() {
        let record: T = result?;
        records.push(record);
    }

    Ok(records)
}

pub fn write_to_file<T: Serialize>(path: &str, records: &[T]) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(path)?;

    for record in records {
        wtr.serialize(record)?;
    }

    wtr.flush()?;
    Ok(())
}
