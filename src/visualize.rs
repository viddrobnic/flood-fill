use std::path::Path;

use image::{Rgba, RgbaImage};

use crate::{Area, Bounds, Point};

const IMG_SCALE: u32 = 10;

pub fn visualize(
    home: &Point,
    points: &[Point],
    output_path: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let area = Area::from_points(points);
    let bounds = Bounds {
        min_x: area.center.x - area.radius,
        min_y: area.center.y - area.radius,
        max_x: area.center.x + area.radius,
        max_y: area.center.y + area.radius,
    };

    let img_width = (bounds.width().ceil() as u32) / IMG_SCALE + 1;
    let img_height = (bounds.height().ceil() as u32) / IMG_SCALE + 1;

    let mut img = RgbaImage::new(img_width, img_height);
    for x in 0..img_width {
        for y in 0..img_height {
            img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
        }
    }

    for p in points {
        let (x, y) = get_pixel_coords(p, &bounds, img_height);
        img.put_pixel(x, y, Rgba([0, 0, 255, 150]));
    }

    let (x, y) = get_pixel_coords(home, &bounds, img_height);
    for dx in 0..10 {
        for dy in 0..10 {
            img.put_pixel(x + dx, y + dy, Rgba([255, 0, 0, 255]));
        }
    }

    img.save(output_path)?;
    Ok(())
}

fn get_pixel_coords(point: &Point, bounds: &Bounds<f32>, height: u32) -> (u32, u32) {
    (
        (point.x - bounds.min_x) as u32 / IMG_SCALE,
        height - ((point.y - bounds.min_y) as u32 / IMG_SCALE),
    )
}
