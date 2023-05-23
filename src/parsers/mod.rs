use std::{ffi::OsStr, path::Path};

pub mod csv;
pub mod excel;

pub(crate) const EXTENSIONS: [&str; 3] = ["csv", "xlsx", "xls"];

pub fn parse_spreadsheets_from_folder<T, P>(path: P) -> anyhow::Result<Vec<T>>
where
    P: AsRef<Path>,
    T: for<'de> serde::Deserialize<'de>,
{
    let directory_elements = std::fs::read_dir(path)?.collect::<Result<Vec<_>, _>>()?;

    let files = directory_elements.into_iter().filter(|entry| {
        let is_file = entry.file_type().map(|ft| ft.is_file()).unwrap_or(false);
        let has_spreadsheet_extension = entry
            .path()
            .extension()
            .map_or(false, |ext| EXTENSIONS.contains(&ext.to_str().unwrap()));

        is_file && has_spreadsheet_extension
    });

    let records = files
        .into_iter()
        .map(|f| match f.path().extension().and_then(OsStr::to_str) {
            Some("csv") => csv::read::<T, _>(f.path()),
            Some("xlsx" | "xls") => excel::read::<T, _>(f.path()),
            _ => Err(anyhow::anyhow!("Invalid file extension")),
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    Ok(records)
}
