use std::fs;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

use anyhow::{anyhow, bail};

use crate::Point;

struct PointWriter<W: io::Write>(W);

impl<W: io::Write> PointWriter<W> {
    fn write(&mut self, point: &Point) -> anyhow::Result<()> {
        let buf = point.x.to_le_bytes();
        self.0.write_all(&buf)?;

        let buf = point.y.to_le_bytes();
        self.0.write_all(&buf)?;

        let buf = point.z.to_le_bytes();
        self.0.write_all(&buf)?;

        Ok(())
    }
}

struct PointReader<R: io::Read>(R);

impl<R: io::Read> PointReader<R> {
    fn read(&mut self) -> anyhow::Result<Vec<Point>> {
        let mut points = vec![];

        let mut comps = [0f32; 3];
        let mut comp_idx = 0;
        let mut buf = [0u8; 4];

        loop {
            match self.0.read_exact(&mut buf) {
                Ok(_) => {
                    let comp = f32::from_le_bytes(buf);
                    comps[comp_idx] = comp;
                    comp_idx += 1;

                    if comp_idx == 3 {
                        points.push(Point {
                            x: comps[0],
                            y: comps[1],
                            z: comps[2],
                        });
                        comp_idx = 0;
                    }
                }

                // We reached EOF and can break
                Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => break,
                // Handle other errors
                Err(err) => bail!(err),
            }
        }

        Ok(points)
    }
}

pub fn read(data_path: PathBuf) -> anyhow::Result<Vec<Point>> {
    let file = fs::File::open(data_path)?;
    let reader = io::BufReader::new(file);

    let mut reader = PointReader(reader);
    reader.read()
}

pub fn import(input: PathBuf, output: PathBuf) -> anyhow::Result<()> {
    // Create output writer
    let out_file = fs::File::create(&output)?;
    let writer = io::BufWriter::new(out_file);
    let mut writer = PointWriter(writer);
    import_rec(&input, &mut writer)?;

    Ok(())
}

fn import_rec<W: io::Write>(input: &PathBuf, writer: &mut PointWriter<W>) -> anyhow::Result<()> {
    println!("Processing directory: {:?}", input);

    let entries = fs::read_dir(input)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            import_rec(&path, writer)?;
        } else {
            import_file(&path, writer)?;
        }
    }

    Ok(())
}

fn import_file<W: io::Write>(input: &PathBuf, writer: &mut PointWriter<W>) -> anyhow::Result<()> {
    // TODO: Ignore non xyz files

    let file = fs::File::open(input)?;
    let mut reader = io::BufReader::new(file);

    let mut buf = String::new();
    loop {
        buf.clear();
        let bytes = reader.read_line(&mut buf)?;
        if bytes == 0 {
            break;
        }

        let mut iter = buf.split_whitespace().filter_map(|s| s.parse::<f32>().ok());
        let arr: [_; 3] = std::array::from_fn(|_| iter.next());

        writer.write(&Point {
            x: arr[0].ok_or(anyhow!("Expected 3 components, got 0"))?,
            y: arr[1].ok_or(anyhow!("Expected 3 components, got 1"))?,
            z: arr[2].ok_or(anyhow!("Expected 3 components, got 2"))?,
        })?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Point;

    use super::{PointReader, PointWriter};

    #[test]
    fn point_read_write() {
        let mut buf = [0u8; 4 * 3 * 2]; // 4 bytes per f32 * 3 f32 per point * 2 points
        let mut writer = PointWriter(&mut buf[..]);

        // Write points
        let points = vec![
            Point {
                x: 0.5,
                y: 1.0,
                z: -1.2,
            },
            Point {
                x: 2.0,
                y: 3.0,
                z: -4.0,
            },
        ];
        for p in &points {
            writer.write(p).unwrap();
        }

        // Read points
        let mut reader = PointReader(&buf[..]);
        let got_points = reader.read().unwrap();

        assert_eq!(points, got_points);
    }
}
