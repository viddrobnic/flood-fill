pub mod query;
pub mod visualize;

mod point;

use std::ops::Sub;

use hribovje::Area;

pub use point::*;

#[derive(Debug)]
pub struct Bounds<T> {
    pub min_x: T,
    pub min_y: T,
    pub max_x: T,
    pub max_y: T,
}

impl<T: Sub<Output = T> + Copy> Bounds<T> {
    pub fn width(&self) -> T {
        self.max_x - self.min_x
    }

    pub fn height(&self) -> T {
        self.max_y - self.min_y
    }
}

impl From<&Area> for Bounds<f32> {
    fn from(area: &Area) -> Self {
        Self {
            min_x: area.center.x - area.radius,
            min_y: area.center.y - area.radius,
            max_x: area.center.x + area.radius,
            max_y: area.center.y + area.radius,
        }
    }
}
