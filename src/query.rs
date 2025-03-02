use std::path::PathBuf;

use crate::data;

pub fn query(_lat: f32, _lon: f32, data_path: PathBuf) -> anyhow::Result<()> {
    let points = data::read(data_path)?;
    println!("Read {} points", points.len());

    Ok(())
}
