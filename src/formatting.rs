#[allow(clippy::trivially_copy_pass_by_ref)]
#[must_use]
pub fn euro_to_string(value: &f32) -> String {
    format!("€{:.2}", value)
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[must_use]
pub fn kwh_to_string(value: &f32) -> String {
    format!("{:.2}kWh", value / 1000.0)
}
