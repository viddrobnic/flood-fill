use crate::{Area, Point, qtree::QTree};

const MAX_DISTANCE: f32 = 26_000.0; // 26km
const JUMP_DISTANCE: f32 = 6.0; // 6m

pub fn query(
    home: &Point,
    points: &[Point],
    depth: f32,
    verbose: bool,
) -> anyhow::Result<Vec<Point>> {
    // Filter points by MAX_DISTANCE and get height of home.
    let mut filtered_points = vec![];
    let mut min_distance = f32::MAX;
    let mut height = 0.0;

    for p in points {
        let dist = p.distance_sq(home);
        if dist > MAX_DISTANCE * MAX_DISTANCE {
            continue;
        }

        filtered_points.push(p.clone());

        if dist < min_distance {
            min_distance = dist;
            height = p.z;
        }
    }

    if verbose {
        println!(
            "[INFO] Filtered out points, left: {}",
            filtered_points.len()
        );
        println!("[INFO] Found nearest height: {}", height);
    }

    // Construct the tree only with points that have correct height..
    let area = Area::from_points(&filtered_points);
    let mut tree = QTree::new(area);
    let mut count = 0;
    for p in filtered_points {
        if p.z < height + depth {
            tree.insert(p)?;
            count += 1;
        }
    }
    if verbose {
        println!("[INFO] Filtered points by height, left: {count}");
        println!("[INFO] Constructed tree, #points: {}", tree.size());
    }

    // Buffer for storing results without allocation.
    let mut buffer: [_; 50] = std::array::from_fn(|_| Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });

    // Get points that can be reached from the user specified point,
    // without being too high. Note: height is already filtered out.
    let mut results = vec![home.clone()];
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
    if verbose {
        println!("[INFO] Got results, #points: {}", results.len());
    }

    Ok(results)
}
