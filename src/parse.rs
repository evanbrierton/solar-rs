use std::path::Path;

use serde::{Deserialize, Serialize};

/// # Errors
///
/// Will return `Err` if `path` does not exist, if the user does not have
/// permission to read it or if the file is not a valid CSV file.
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

/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to write to it.
pub fn write_to_file<T, P>(path: P, records: &[T]) -> Result<(), csv::Error>
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
