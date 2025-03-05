use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use flood_fill::{LatLon, Point, data, query, visualize::visualize};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Imports raw text data to binary data.
    Import {
        /// Path to input directory.
        #[arg(long)]
        input: PathBuf,

        /// Path to output file
        #[arg(long)]
        output: PathBuf,
    },

    /// Query data for a lat long point
    Simulate {
        #[arg(long)]
        lat: f32,

        #[arg(long)]
        lon: f32,

        /// Path to data file
        #[arg(long)]
        data: PathBuf,

        #[arg(long, default_value = "flood.png")]
        output: PathBuf,

        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Import { input, output } => data::import(input, output),
        Command::Simulate {
            lat,
            lon,
            data,
            output,
            verbose,
        } => {
            let home: Point = LatLon::new(lat, lon).try_into()?;
            simulate(home, data, output, verbose)
        }
    }
}

fn simulate<P: AsRef<Path> + Debug>(
    home: Point,
    data: P,
    output: P,
    verbose: bool,
) -> anyhow::Result<()> {
    let points = data::read(data)?;
    if verbose {
        println!("[INFO] Read data, #points: {}", points.len());
    }

    let points = query::query(&home, &points, verbose)?;
    visualize(&home, &points, &output)?;

    // Nicer formatting of success message
    if verbose {
        println!();
    }
    println!("Image saved to: {:?}", output);
    Ok(())
}
