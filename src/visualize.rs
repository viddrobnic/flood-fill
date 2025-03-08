use std::{fs, io::Write, path::Path};

use askama::Template;
use hribovje::Point;
use image::{Rgba, RgbaImage};

use crate::{Area, Bounds, LatLon};

const IMG_SCALE: u32 = 10;

pub fn visualize(
    home: &Point,
    points: &[Point],
    output_path: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let area = Area::from_points(points);
    let bounds = Bounds::from(&area);

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

#[derive(Template)]
#[template(path = "visualize.html")]
struct VisualizeMapTemplate {
    home: LatLon,
    nw: LatLon,
    se: LatLon,
}

pub fn render_html(
    home: &Point,
    points: &[Point],
    output_path: impl AsRef<Path>,
) -> anyhow::Result<()> {
    // Construct points
    let area = Area::from_points(points);
    let bounds = Bounds::from(&area);

    let nw = Point {
        x: bounds.min_x,
        y: bounds.min_y,
        z: 0.0,
    };
    let nw: LatLon = nw.try_into()?;

    let se = Point {
        x: bounds.max_x,
        y: bounds.max_y,
        z: 0.0,
    };
    let se: LatLon = se.try_into()?;

    // Create template
    let map_templ = VisualizeMapTemplate {
        home: home.clone().try_into()?,
        nw,
        se,
    };

    let map_render = map_templ.render()?;

    let mut file = fs::File::create(output_path)?;
    file.write_all(map_render.as_bytes())?;

    Ok(())
}

fn get_pixel_coords(point: &Point, bounds: &Bounds<f32>, height: u32) -> (u32, u32) {
    (
        (point.x - bounds.min_x) as u32 / IMG_SCALE,
        height - ((point.y - bounds.min_y) as u32 / IMG_SCALE),
    )
}
