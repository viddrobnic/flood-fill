use proj::{Proj, ProjError};

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LatLon {
    pub lat: f32,
    pub lon: f32,
}

impl Point {
    pub fn distance_sq(&self, other: &Point) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        dx * dx + dy * dy
    }

    pub fn distance(&self, other: &Point) -> f32 {
        self.distance_sq(other).sqrt()
    }
}

impl LatLon {
    pub fn new(lat: f32, lon: f32) -> Self {
        Self { lat, lon }
    }
}

impl TryFrom<LatLon> for Point {
    type Error = ProjError;

    fn try_from(value: LatLon) -> Result<Self, Self::Error> {
        let wgs84_to_d96tm = Proj::new_known_crs("EPSG:4326", "EPSG:3794", None)
            .expect("Failed to load coord converter");

        let (x, y) = wgs84_to_d96tm.convert((value.lon, value.lat))?;
        Ok(Self { x, y, z: 0.0 })
    }
}

impl TryFrom<Point> for LatLon {
    type Error = ProjError;

    fn try_from(value: Point) -> Result<Self, Self::Error> {
        let d96tm_to_wgs84 = Proj::new_known_crs("EPSG:3794", "EPSG:4326", None)
            .expect("Failed to load coord converter");

        let (lon, lat) = d96tm_to_wgs84.convert((value.x, value.y))?;
        Ok(Self { lon, lat })
    }
}
