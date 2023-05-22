use solar_rs::solar_data::SolarData;
use std::fs::read_dir;

fn main() {
    let dir = read_dir("data").unwrap();
    let data = SolarData::from(dir);

    println!("{}", data)
}
