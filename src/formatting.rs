pub const EURO: char = '\u{20AC}';

#[allow(clippy::trivially_copy_pass_by_ref)]
#[must_use]
pub(crate) fn euro_to_string(value: &f64) -> String {
    format!("{EURO}{value:.2}")
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[must_use]
pub(crate) fn watt_hour_to_string(value: &f64) -> String {
    match value {
        x if *x < 1000_f64 => format!("{value:.2}Wh"),
        x if *x < 1_000_000_f64 => format!("{:.2}kWh", value / 1000_f64),
        x if *x < 1_000_000_000_f64 => format!("{:.2}MWh", value / 1_000_000_f64),
        _ => format!("{:.2}GWh", value / 1_000_000_000_f64),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{anyhow, ensure};

    #[test]
    fn test_euro_to_string() -> anyhow::Result<()> {
        let value = 123.45_f64;

        ensure!(
            euro_to_string(&value) == "€123.45",
            anyhow!("Expected €123.45, got {}", euro_to_string(&value))
        );

        Ok(())
    }

    #[test]
    fn test_kwh_to_string() -> anyhow::Result<()> {
        let value = 123.45_f64;

        ensure!(
            watt_hour_to_string(&value) == "123.45Wh",
            anyhow!("Expected 123.45Wh, got {}", watt_hour_to_string(&value))
        );

        let value = 12_345.67_f64;

        ensure!(
            watt_hour_to_string(&value) == "12.35kWh",
            anyhow!("Expected 12.35kWh, got {}", watt_hour_to_string(&value))
        );

        let value = 12_345_678.90_f64;

        ensure!(
            watt_hour_to_string(&value) == "12.35MWh",
            anyhow!("Expected 12.35MWh, got {}", watt_hour_to_string(&value))
        );

        let value = 12_345_678_901.23_f64;

        ensure!(
            watt_hour_to_string(&value) == "12.35GWh",
            anyhow!("Expected 12.35GWh, got {}", watt_hour_to_string(&value))
        );

        Ok(())
    }
}
