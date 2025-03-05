pub mod data;
pub mod query;
pub mod visualize;

mod area;
mod point;
mod qtree;

use std::ops::Sub;

pub use area::*;
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
