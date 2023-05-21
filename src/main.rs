use solar_rs::{parse::read_from_file, serialization::SolarRecord};

fn main() {
    let info = read_from_file::<SolarRecord>("data/20230520.csv");

    match info {
        Ok(records) => {
            for record in records {
                println!("{:?}", record);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
