use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
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
    x: f32,
    y: f32,
    z: f32,
    real: f32,
    imag: f32,
}

/// Convert stdin (CSV) to output (SB)
fn convert(meta: &PathBuf, data: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());

    println!(
        "Writing to {:?} (text metadata) and {:?} (binary data).",
        meta, data
    );
    let mut meta = File::create_new(meta)?;
    let mut data = File::create_new(data)?;

    // 1. Write basic data header

    // SBF header flag
    data.write_all(&[42, 42])?;
    // Point count
    // Fill a zero first, and write `n_point` later.
    data.write_all(&0_u64.to_be_bytes())?;
    // Scalar field count, (Re, Im) ⇒ 2
    data.write_all(&2_u16.to_be_bytes())?;

    // Coordinate shifts of x,y,z
    // Our data is small enough, and therefore we do not need to shift.
    let shifts: [f64; 3] = [0., 0., 0.];
    data.write_all(
        &shifts
            .iter()
            .flat_map(|s| s.to_be_bytes())
            .collect::<Vec<u8>>(),
    )?;

    // Fill 36-63 with zeros
    data.write_all(&[0_u8; 64 - 36])?;

    // 2. Write data body

    let mut n_point: u64 = 0;
    for point in reader.deserialize() {
        let point: Point = point?;

        data.write_all(&point.x.to_be_bytes())?;
        data.write_all(&point.y.to_be_bytes())?;
        data.write_all(&point.z.to_be_bytes())?;
        data.write_all(&point.real.to_be_bytes())?;
        data.write_all(&point.imag.to_be_bytes())?;

        n_point += 1;
    }

    // 3. Finish data header

    // Write point count after SBF header flag
    data.seek(SeekFrom::Start(2))?;
    data.write_all(&n_point.to_be_bytes())?;

    // 4. Write meta

    writeln!(
        meta,
        "[SBF]
Points={n_point}
GlobalShift={:.6}, {:.6}, {:.6}
SFCount=2
SF1=real
SF2=imag
",
        shifts[0], shifts[1], shifts[2]
    )?;

    println!("Successfully read and convert {n_point} points.");

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    // Validate
    assert_eq!(cli.output.extension().unwrap(), "sbf");
    let data_output = cli.output.with_extension("sbf.data");

    // Execute
    if let Err(err) = convert(&cli.output, &data_output) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
