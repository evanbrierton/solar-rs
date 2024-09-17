use std::path::Path;

use serde::Deserialize;

/// Reads a CSV file from the given path and returns a vector of deserialized
/// records.
///
/// # Errors
///
/// Will return `Err` if `path` does not exist, if the user does not have
/// permission to read it or if the file is not a valid CSV file.
///
/// # Examples
///
/// ```
// / use serde::Deserialize;
// / use solar_rs::parsers::csv::read;
// /
// / #[derive(Deserialize)]
// / struct Record {
// /    name: String,
// /    age: u8,
// / }
// /
// / let records = read::<Record, _>("data.csv").unwrap();
// / assert_eq!(records.len(), 3);
/// ```
///
/// The CSV file is expected to have a header row that specifies the names of
/// the fields in each record.
pub fn read<T, P>(path: P) -> anyhow::Result<Vec<T>>
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

pub fn write<T, P>(path: P, records: &[T]) -> anyhow::Result<()>
where
    T: serde::Serialize,
    P: AsRef<Path>,
{
    let mut wtr = csv::Writer::from_path(path)?;

    for record in records {
        wtr.serialize(record)?;
    }

    wtr.flush()?;

    Ok(())
}
