use solar_rs::solar_data::SolarData;
use std::fs::read_dir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("Usage: solar-rs <path>");
        return Ok(());
    }

    let dir = read_dir(&args[1]).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => format!("{}: No such file or directory", &args[1]),
        _ => format!("{}: {}", &args[1], e),
    })?;

    let data = SolarData::from(dir);

    println!("{}", data);
    Ok(())
}
