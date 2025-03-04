use std::path::PathBuf;

use crate::{
    LatLon, Point, data,
    qtree::{Area, QTree},
};

const MAX_DISTANCE: f32 = 10_000.0; // 10km

const JUMP_DISTANCE: f32 = 6.0; // 6m
const HEIGHT_DIST: f32 = 1.0; //1m

pub fn query(point: LatLon, data_path: PathBuf) -> anyhow::Result<()> {
    let mut points = data::read(data_path)?;
    let point: Point = point.try_into()?;
    println!("Read data, #points: {}", points.len());

    points.retain(|p| p.distance_sq(&point) <= MAX_DISTANCE * MAX_DISTANCE);
    println!("Filtered out points, left: {}", points.len());

    // Find closes point to get height
    let (_, height) = points.iter().fold((f32::MAX, 0.0), |(min_d, h), p| {
        let d = p.distance_sq(&point);
        if d < min_d { (d, p.z) } else { (min_d, h) }
    });
    println!("Found nearest height: {}", height);

    // Filter points by height
    points.retain(|p| p.z <= height + HEIGHT_DIST);
    println!("Filtered points by height, #left: {}", points.len());

    // Construct a tree
    let area = area_from_points(&points);
    let mut tree = QTree::new(area);
    for p in points {
        tree.insert(p)?;
    }
    println!("Constructed tree, #points: {}", tree.size());

    // Buffer for storing results without allocation.
    let mut buffer: [_; 50] = std::array::from_fn(|_| Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });

    // Get points that can be reached from the user specified point,
    // without being too high. Note: height is already filtered out.
    let mut results = vec![point];
    let mut idx = 0;
    while idx < results.len() {
        let center = &results[idx];
        idx += 1;

        let area = Area {
            center: center.clone(),
            radius: JUMP_DISTANCE,
        };

        let nr_neigh = tree.query(&area, &mut buffer)?;
        for neigh in &buffer[..nr_neigh] {
            results.push(neigh.clone())
        }
    }
    println!("Got results, #points: {}", results.len());

    // TODO: Display this

    Ok(())
}

fn area_from_points(points: &[Point]) -> Area {
    let (min_x, min_y, max_x, max_y) = points.iter().fold(
        (f32::MAX, f32::MAX, f32::MIN, f32::MIN),
        |(min_x, min_y, max_x, max_y), p| {
            (
                min_x.min(p.x),
                min_y.min(p.y),
                max_x.max(p.x),
                max_y.max(p.y),
            )
        },
    );

    let width = max_x - min_x;
    let height = max_y - min_y;

    Area {
        center: Point {
            x: width / 2.0 + min_x,
            y: height / 2.0 + min_y,
            z: 0.0,
        },
        radius: width.max(height) / 2.0,
    }
}
