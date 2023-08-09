use std::path::Path;

use calamine::{RangeDeserializerBuilder, Reader, Xlsx};
use serde::Deserialize;

/// Reads an Excel file from the given path and returns a vector of deserialized
/// records.
///
/// # Errors
///
/// Will return `Err` if `path` does not exist, if the user does not have
/// permission to read it or if the file is not a valid Excel file.
///
pub fn read<T, P>(path: P) -> anyhow::Result<Vec<T>>
where
    T: for<'de> Deserialize<'de>,
    P: AsRef<Path>,
{
    let mut workbook: Xlsx<_> = calamine::open_workbook(path)?;
    let mut records = Vec::new();

    let range = workbook
        .worksheet_range("Sheet1")
        .ok_or(calamine::Error::Msg("Sheet not found"))??;

    let iter: calamine::RangeDeserializer<calamine::DataType, T> =
        RangeDeserializerBuilder::new().from_range(&range)?;

    for result in iter {
        let record: T = result?;
        records.push(record);
    }

    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{ensure, Context};
    use tempfile::NamedTempFile;
    use xlsxwriter::{Format, Workbook};

    #[derive(Debug, Deserialize, PartialEq)]
    struct Record {
        name: String,
        age: u8,
    }

    #[test]
    fn test_read_excel() -> anyhow::Result<()> {
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

        let file = NamedTempFile::new()?;
        let path = file.path().to_str().context("Failed to get path")?;

        let workbook = Workbook::new(path)?;
        let mut worksheet = workbook.add_worksheet(None)?;

        let mut bold = Format::new();
        bold.set_bold();

        worksheet.write_string(0, 0, "name", Some(&bold)).ok();
        worksheet.write_string(0, 1, "age", Some(&bold)).ok();
        worksheet.write_string(1, 0, "Alice", None).ok();
        worksheet.write_number(1, 1, 25.0, None).ok();
        worksheet.write_string(2, 0, "Bob", None).ok();
        worksheet.write_number(2, 1, 30.0, None).ok();

        workbook.close().ok();

        let records = read::<Record, _>(file.path())?;
        ensure!(records == expected, "Records do not match");

        Ok(())
    }

    #[test]
    fn test_read_excel_invalid_file() -> anyhow::Result<()> {
        let result = read::<Record, _>("/path/to/invalid/file.xlsx");

        ensure!(result.is_err(), "Expected error");
        Ok(())
    }
}
