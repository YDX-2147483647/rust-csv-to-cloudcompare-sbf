use std::{error::Error, io, process};

#[derive(Debug, serde::Deserialize)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
    real: f64,
    imag: f64,
}

fn example() -> Result<(), Box<dyn Error>> {
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
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
