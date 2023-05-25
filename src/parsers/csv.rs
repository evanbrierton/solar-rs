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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::ensure;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Record {
        name: String,
        age: u8,
    }

    #[test]
    fn test_read_csv() -> anyhow::Result<()> {
        let expected = vec![
            Record {
                name: "Alice".to_owned(),
                age: 25,
            },
            Record {
                name: "Bob".to_owned(),
                age: 30,
            },
        ];

        let mut file = NamedTempFile::new()?;
        writeln!(file, "name,age").ok();
        writeln!(file, "Alice,25").ok();
        writeln!(file, "Bob,30").ok();
        file.flush().ok();

        let records = read::<Record, _>(file.path())?;
        ensure!(records == expected, "Records do not match");

        Ok(())
    }

    #[test]
    fn test_read_csv_invalid_file() -> anyhow::Result<()> {
        let result = read::<Record, _>("/path/to/invalid/file.csv");
        ensure!(result.is_err());

        Ok(())
    }
}
