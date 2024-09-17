use clap::Parser;
use solar_rs::{period::Period, solar_data::SolarData};

#[derive(Parser, Debug)]
struct Args {
    path: String,
    #[arg(value_name = "OUTPUT", conflicts_with = "output_flag")]
    output_positional: Option<String>,
    #[arg(
        long = "output",
        short,
        value_name = "OUTPUT",
        conflicts_with = "output_positional"
    )]
    output_flag: Option<String>,

    #[arg(short, long, value_enum, default_value_t = Period::Month, value_parser = clap::value_parser!(Period))]
    period: Period,

    #[arg(short, long, default_value = "11000")]
    cost: f64,

    #[arg(short, long, default_value = "12")]
    limit: usize,
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let output = args.output_positional.or(args.output_flag);

    let data = SolarData::from_folder(args.path, args.period, args.cost, args.limit)?;

    if let Some(output) = output {
        data.write(output)?;
        return Ok(());
    }

    println!("{}", data);
    Ok(())
}
