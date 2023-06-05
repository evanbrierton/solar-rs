#[allow(clippy::trivially_copy_pass_by_ref)]
#[must_use]
pub fn euro_to_string(value: &f64) -> String {
    format!("€{:.2}", value)
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[must_use]
pub fn kwh_to_string(value: &f64) -> String {
    format!("{:.2}kWh", value / 1000_f64)
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
        let value = 12_345.67_f64;

        ensure!(
            kwh_to_string(&value) == "12.35kWh",
            anyhow!("Expected 12.35kWh, got {}", kwh_to_string(&value))
        );

        Ok(())
    }
}
