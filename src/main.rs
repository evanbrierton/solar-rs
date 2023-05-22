use solar_rs::solar_data::SolarData;

fn main() -> anyhow::Result<()> {
    let data = SolarData::from_folder("data")?;

    println!("{}", data);

    Ok(())
}
