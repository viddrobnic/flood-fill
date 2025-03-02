use std::path::PathBuf;

use clap::{Parser, Subcommand};
use flood_fill::{data, query};

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
    Query {
        #[arg(long)]
        lat: f32,

        #[arg(long)]
        lon: f32,

        /// Path to data file
        #[arg(long)]
        data: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Import { input, output } => data::import(input, output),
        Command::Query { lat, lon, data } => query::query(lat, lon, data),
    }
}
