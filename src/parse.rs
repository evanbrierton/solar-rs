use std::{error::Error, path::Path};

use serde::{Deserialize, Serialize};

pub fn read_from_file<T, P>(path: P) -> Result<Vec<T>, csv::Error>
where
    T: for<'de> Deserialize<'de>,
    P: AsRef<Path>,
{
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records = Vec::new();

    for result in rdr.deserialize() {
        let record: T = result?;
        records.push(record);
    }

    Ok(records)
}

pub fn write_to_file<T, P>(path: &str, records: &[T]) -> Result<(), Box<dyn Error>>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let mut wtr = csv::Writer::from_path(path)?;

    for record in records {
        wtr.serialize(record)?;
    }

    wtr.flush()?;
    Ok(())
}
