use std::path::Path;

use calamine::{RangeDeserializerBuilder, Reader};
use serde::Deserialize;

/// # Errors
///
/// Will return `Err` if `path` does not exist, if the user does not have
/// permission to read it or if the file is not a valid excel file.
pub fn read<T, P>(path: P) -> anyhow::Result<Vec<T>>
where
    T: for<'de> Deserialize<'de>,
    P: AsRef<Path>,
{
    let mut workbook = calamine::open_workbook_auto(path)?;
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
    use std::io::Write;
    use tempfile::NamedTempFile;
    use xlsxwriter::{Format, Workbook, Worksheet};

    #[derive(Debug, Deserialize, PartialEq)]
    struct Record {
        name: String,
        age: u8,
    }

    #[test]
    fn test_read_excel() {
        let mut file = NamedTempFile::new().unwrap();
        let workbook = Workbook::new(file.path().to_str().unwrap()).unwrap();


        let mut worksheet = match workbook.add_worksheet(None) {
            Ok(worksheet) => worksheet,
            Err(e) => panic!("Failed to add worksheet: {}", e),
        };

        let bold = workbook.add_format().set_bold(true);
        worksheet.write_string(0, 0, "name", Some(&bold)).unwrap();
        worksheet.write_string(0, 1, "age", Some(&bold)).unwrap();
        worksheet.write_string(1, 0, "Alice", None).unwrap();
        worksheet.write_number(1, 1, 25, None).unwrap();
        worksheet.write_string(2, 0, "Bob", None).unwrap();
        worksheet.write_number(2, 1, 30, None).unwrap();

        workbook.close().unwrap();

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
    fn test_read_excel_invalid_file() {
        let result = read::<Record, _>("/path/to/invalid/file.xlsx");
        assert!(result.is_err());
    }
}