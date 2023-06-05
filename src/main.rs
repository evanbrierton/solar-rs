use solar_rs::solar_data::SolarData;

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("Usage: solar-rs <path>");
        return Ok(());
    }

    let data = SolarData::from_folder(&args[1])?;

    println!("{}", data);

    Ok(())
}
