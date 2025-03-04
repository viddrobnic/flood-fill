//! A very specific implementation of quad tree,
//! that allows for zero allocation query and removal of points in
//! a single operation.

use anyhow::{anyhow, bail};

use crate::Point;

// Max points in leaf node
const MAX_POINTS: usize = 1000;

#[derive(Debug)]
enum NodeInner {
    Leaf {
        points: Vec<Point>,
    },
    Intermediate {
        nw: Box<Node>,
        ne: Box<Node>,
        sw: Box<Node>,
        se: Box<Node>,
    },
}

#[derive(Debug)]
struct Node {
    area: Area,
    inner: NodeInner,
}

/// Square are with width = height = 2 * radius
#[derive(Debug)]
pub struct Area {
    pub center: Point,
    pub radius: f32,
}

pub struct QTree(Node);

impl QTree {
    pub fn new(area: Area) -> Self {
        Self(Node {
            area,
            inner: NodeInner::Leaf { points: vec![] },
        })
    }

    pub fn insert(&mut self, point: Point) -> anyhow::Result<()> {
        self.0.insert(point)
    }

    pub fn size(&self) -> usize {
        self.0.size()
    }

    pub fn query(&mut self, area: &Area, results: &mut [Point]) -> anyhow::Result<usize> {
        let mut idx = 0;
        self.0.query(area, results, &mut idx)?;
        Ok(idx)
    }
}

impl Node {
    fn insert(&mut self, point: Point) -> anyhow::Result<()> {
        if !self.area.is_point_inside(&point) {
            bail!("Point outside of node area");
        }

        match &mut self.inner {
            NodeInner::Intermediate { nw, ne, sw, se } => {
                if nw.area.is_point_inside(&point) {
                    nw.insert(point)
                } else if ne.area.is_point_inside(&point) {
                    ne.insert(point)
                } else if sw.area.is_point_inside(&point) {
                    sw.insert(point)
                } else if se.area.is_point_inside(&point) {
                    se.insert(point)
                } else {
                    Err(anyhow!("Invalid tree"))
                }
            }
            NodeInner::Leaf { points } => {
                points.push(point);
                if points.len() > MAX_POINTS {
                    self.subdivide();
                }

                Ok(())
            }
        }
    }

    fn query(&mut self, area: &Area, results: &mut [Point], idx: &mut usize) -> anyhow::Result<()> {
        if !self.area.intersects(area) {
            bail!("Area outside of node area");
        }

        match &mut self.inner {
            NodeInner::Intermediate { nw, ne, sw, se } => {
                if nw.area.intersects(area) {
                    nw.query(area, results, idx)?;
                }
                if ne.area.intersects(area) {
                    ne.query(area, results, idx)?;
                }
                if sw.area.intersects(area) {
                    sw.query(area, results, idx)?;
                }
                if se.area.intersects(area) {
                    se.query(area, results, idx)?;
                }
            }
            NodeInner::Leaf { points } => {
                let mut i = 0;
                while i < points.len() {
                    if !area.is_point_inside(&points[i]) {
                        i += 1;
                        continue;
                    }

                    let point = points.swap_remove(i);
                    results[*idx] = point;
                    *idx += 1;
                }
            }
        }

        Ok(())
    }

    fn new_leaf(area: Area) -> Self {
        Self {
            area,
            inner: NodeInner::Leaf { points: vec![] },
        }
    }

    fn subdivide(&mut self) {
        let area = &self.area;
        let r = area.radius / 2.0;

        let nw_area = Area {
            center: Point {
                x: area.center.x - r,
                y: area.center.y - r,
                z: 0.0,
            },
            radius: r,
        };
        let ne_area = Area {
            center: Point {
                x: area.center.x + r,
                y: area.center.y - r,
                z: 0.0,
            },
            radius: r,
        };
        let sw_area = Area {
            center: Point {
                x: area.center.x - r,
                y: area.center.y + r,
                z: 0.0,
            },
            radius: r,
        };
        let se_area = Area {
            center: Point {
                x: area.center.x + r,
                y: area.center.y + r,
                z: 0.0,
            },
            radius: r,
        };

        let mut curr_leaf = NodeInner::Intermediate {
            nw: Box::new(Node::new_leaf(nw_area)),
            ne: Box::new(Node::new_leaf(ne_area)),
            sw: Box::new(Node::new_leaf(sw_area)),
            se: Box::new(Node::new_leaf(se_area)),
        };
        std::mem::swap(&mut curr_leaf, &mut self.inner);

        let NodeInner::Leaf { points } = curr_leaf else {
            panic!("subdivide called on non-leaf node");
        };
        for p in points {
            self.insert(p).expect("subdivide became invalid");
        }
    }

    fn size(&self) -> usize {
        match &self.inner {
            NodeInner::Leaf { points } => points.len(),
            NodeInner::Intermediate { nw, ne, sw, se } => {
                nw.size() + ne.size() + sw.size() + se.size()
            }
        }
    }
}

impl Area {
    pub fn is_point_inside(&self, point: &Point) -> bool {
        let x_inside =
            point.x >= self.center.x - self.radius && point.x <= self.center.x + self.radius;
        let y_inside =
            point.y >= self.center.y - self.radius && point.y <= self.center.y + self.radius;

        x_inside && y_inside
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let dx = (self.center.x - other.center.x).abs();
        let dy = (self.center.y - other.center.y).abs();

        let x_inter = dx <= self.radius + other.radius;
        let y_inter = dy <= self.radius + other.radius;
        x_inter && y_inter
    }
}

#[cfg(test)]
mod tests {
    use crate::Point;

    use super::Area;

    #[test]
    fn area_intersects() {
        let cases = [
            (
                Area {
                    center: Point {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    radius: 1.0,
                },
                Area {
                    center: Point {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    radius: 1.0,
                },
                true,
            ),
            (
                Area {
                    center: Point {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    radius: 1.0,
                },
                Area {
                    center: Point {
                        x: 2.0,
                        y: 2.0,
                        z: 0.0,
                    },
                    radius: 1.0,
                },
                true,
            ),
            (
                Area {
                    center: Point {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    radius: 1.0,
                },
                Area {
                    center: Point {
                        x: 2.0,
                        y: 2.0,
                        z: 0.0,
                    },
                    radius: 0.9,
                },
                false,
            ),
        ];

        for (a1, a2, expected) in &cases {
            assert_eq!(a1.intersects(a2), *expected);
            assert_eq!(a2.intersects(a1), *expected);
        }
    }
}
