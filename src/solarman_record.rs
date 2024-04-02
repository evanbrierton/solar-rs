use chrono::{DateTime, TimeZone, Utc};
use num_traits::{Num, NumCast};
use serde::{Deserialize, Deserializer};

/// A record of solar power production and consumption
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub(crate) struct SolarmanRecord {
    /// The time at which the record was updated.
    #[serde(rename = "Updated Time", deserialize_with = "deserialize_date")]
    pub time: DateTime<Utc>,
    /// The amount of power being produced, in watts.
    #[serde(
        rename = "Production Power(W)",
        deserialize_with = "deserialize_decimal"
    )]
    pub production: u32,
    /// The amount of power being consumed, in watts.
    #[serde(
        rename = "Consumption Power(W)",
        deserialize_with = "deserialize_decimal"
    )]
    pub consumption: u32,
    /// The amount of power being drawn or fed back into the grid, in watts.
    /// Positive values indicate power being fed back into the grid.
    /// Negative values indicate power being drawn from the grid.
    #[serde(rename = "Grid Power(W)", deserialize_with = "deserialize_decimal")]
    pub grid: i32,
    /// The amount of power being charged or discharged from the battery, in watts.
    /// Positive values indicate power being charged into the battery.
    /// Negative values indicate power being discharged from the battery.
    #[serde(rename = "Battery Power(W)", deserialize_with = "deserialize_decimal")]
    pub battery: i32,
    /// The state of charge of the battery, as a percentage.
    #[serde(rename = "SoC(%)", deserialize_with = "deserialize_decimal")]
    pub soc: u8,
}

/// Deserializes a decimal value from a string.
///
/// This function is used to deserialize decimal values from strings in the
/// `SolarmanRecord` struct.
///
/// # Errors
///
/// Will return an error if the string cannot be parsed as a decimal value.
fn deserialize_decimal<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Num + TryFrom<i32>,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer).map_err(serde::de::Error::custom)?;

    if s.is_empty() {
        return Ok(T::zero());
    }

    let f: f64 = s.parse().map_err(serde::de::Error::custom)?;

    let i: i32 = match NumCast::from(f) {
        Some(x) => x,
        None => return Err(serde::de::Error::custom("Failed to convert to i32")),
    };

    let Ok(d) = T::try_from(i) else {
        return Err(serde::de::Error::custom("Failed to convert to integer type"));
    };

    Ok(d)
}

/// Deserializes a date and time value from a string.
///
/// This function is used to deserialize date and time values from strings in
/// the `SolarmanRecord` struct.
///
/// # Errors
///
/// Will return an error if the string cannot be parsed as a date and time value.
fn deserialize_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    const FORMAT: &str = "%Y/%m/%d %H:%M";
    let s = String::deserialize(deserializer)?;

    Utc.datetime_from_str(&s, FORMAT)
        .map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{ensure, Context};
    use serde::de::{
        value::{self, F64Deserializer, StrDeserializer},
        IntoDeserializer,
    };
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn test_deserialize_decimal() -> anyhow::Result<()> {
        let input = 123.00_f64;
        let expected = 123_i32;

        let deserializer: F64Deserializer<value::Error> = input.into_deserializer();

        let result: i32 = deserialize_decimal(deserializer)?;
        ensure!(result == expected);

        Ok(())
    }

    #[test]
    fn test_deserialize_date() -> anyhow::Result<()> {
        let input = "2023/05/24 01:35";
        let expected = Utc
            .with_ymd_and_hms(2023, 5, 24, 1, 35, 0)
            .single()
            .context("Failed to create expected DateTime<Utc> value")?;

        let deserializer: StrDeserializer<value::Error> = input.into_deserializer();

        let result: DateTime<Utc> = deserialize_date(deserializer)?;
        ensure!(result == expected);

        Ok(())
    }

    #[test]
    fn test_deserialize_solarman_record() -> anyhow::Result<()> {
        let input = [
            Token::Struct {
                name: "SolarmanRecord",
                len: 6,
            },
            Token::Str("Updated Time"),
            Token::Str("2023/05/24 01:35"),
            Token::Str("Production Power(W)"),
            Token::F64(123.00),
            Token::Str("Consumption Power(W)"),
            Token::F64(456.00),
            Token::Str("Grid Power(W)"),
            Token::F64(-789.00),
            Token::Str("Battery Power(W)"),
            Token::F64(1011.00),
            Token::Str("SoC(%)"),
            Token::F64(12.00),
            Token::StructEnd,
        ];

        let time = Utc
            .with_ymd_and_hms(2023, 5, 24, 1, 35, 0)
            .single()
            .context("Failed to create expected DateTime<Utc> value")?;

        let expected = SolarmanRecord {
            time,
            production: 123,
            consumption: 456,
            grid: -789,
            battery: 1011,
            soc: 12,
        };

        assert_de_tokens(&expected, &input);

        Ok(())
    }
}
