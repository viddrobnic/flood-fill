use crate::{Area, Point, qtree::QTree};

const MAX_DISTANCE: f32 = 100_000.0; // 100km

const JUMP_DISTANCE: f32 = 6.0; // 6m

pub fn query(
    home: &Point,
    points: &[Point],
    depth: f32,
    verbose: bool,
) -> anyhow::Result<Vec<Point>> {
    let mut points: Vec<_> = points
        .iter()
        .filter_map(|p| {
            if p.distance_sq(home) <= MAX_DISTANCE * MAX_DISTANCE {
                Some(p.clone())
            } else {
                None
            }
        })
        .collect();

    if verbose {
        println!("[INFO] Filtered out points, left: {}", points.len());
    }

    // Find closes point to get height
    let (_, height) = points.iter().fold((f32::MAX, 0.0), |(min_d, h), p| {
        let d = p.distance_sq(home);
        if d < min_d { (d, p.z) } else { (min_d, h) }
    });
    if verbose {
        println!("[INFO] Found nearest height: {}", height);
    }

    // Filter points by height
    points.retain(|p| p.z <= height + depth);
    if verbose {
        println!("[INFO] Filtered points by height, #left: {}", points.len());
    }

    // Construct a tree
    let area = Area::from_points(&points);
    let mut tree = QTree::new(area);
    for p in points {
        tree.insert(p)?;
    }
    if verbose {
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
