use std::path::PathBuf;
use std::{error::Error, io, process};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long, short)]
    output: PathBuf,
}

#[derive(Debug, serde::Deserialize)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
    real: f64,
    imag: f64,
}

/// Convert stdin (CSV) to output (SB)
fn convert(_meta: &PathBuf, _data: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: Point = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    // Validate
    assert_eq!(cli.output.extension().unwrap(), "sbf");
    let data_output = cli.output.with_extension("sbf.data");
    println!(
        "Write to {:?} (text metadata) and {:?} (binary data).",
        cli.output, data_output
    );

    // Execute
    if let Err(err) = convert(&cli.output, &data_output) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
