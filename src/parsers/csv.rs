use std::path::Path;

use serde::Deserialize;

/// # Errors
///
/// Will return `Err` if `path` does not exist, if the user does not have
/// permission to read it or if the file is not a valid CSV file.
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
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Record {
        name: String,
        age: u8,
    }

    #[test]
    fn test_read_csv() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "name,age").unwrap();
        writeln!(file, "Alice,25").unwrap();
        writeln!(file, "Bob,30").unwrap();
        file.flush().unwrap();

        let records = read::<Record, _>(file.path()).unwrap();
        assert_eq!(
            records,
            vec![
                Record {
                    name: "Alice".to_string(),
                    age: 25,
                },
                Record {
                    name: "Bob".to_string(),
                    age: 30,
                },
            ]
        );
    }

    #[test]
    fn test_read_csv_invalid_file() {
        let result = read::<Record, _>("/path/to/invalid/file.csv");
        assert!(result.is_err());
    }
}