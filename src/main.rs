use std::{
    fmt::Debug,
    fs, io,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use flood_fill::{
    LatLon, query,
    visualize::{render_html, visualize},
};
use hribovje::{Point, data};

const IMAGE_OUTPUT: &str = "flood.png";
const HTML_OUTPUT: &str = "flood.html";

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

        /// Depth of water at the given location, given in meters.
        #[arg(long, default_value_t = 1.0)]
        depth: f32,

        /// Path to data file
        #[arg(long)]
        data: PathBuf,

        /// Print verbose info.
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Import { input, output } => {
            let out_file = fs::File::create(output)?;
            let writer = io::BufWriter::new(out_file);

            data::import_data(input, writer)?;
            Ok(())
        }
        Command::Simulate {
            lat,
            lon,
            depth,
            data,
            verbose,
        } => {
            let home: Point = LatLon::new(lat, lon).try_into()?;
            simulate(home, data, depth, verbose)
        }
    }
}

fn simulate<P: AsRef<Path> + Debug>(
    home: Point,
    data: P,
    depth: f32,
    verbose: bool,
) -> anyhow::Result<()> {
    let in_file = fs::File::open(data)?;
    let reader = io::BufReader::new(in_file);

    let points = data::read_points(reader)?;
    if verbose {
        println!("[INFO] Read data, #points: {}", points.len());
    }

    let points = query::query(&home, &points, depth, verbose)?;
    visualize(&home, &points, IMAGE_OUTPUT)?;

    // Nicer formatting of success message
    if verbose {
        println!();
    }
    println!("Image saved to: {IMAGE_OUTPUT}");

    render_html(&home, &points, HTML_OUTPUT)?;
    println!("HTML saved to: {HTML_OUTPUT}\nOpen it in your browser and play around.");

    Ok(())
}
