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
