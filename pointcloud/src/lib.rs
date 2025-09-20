pub mod types;
use std::path::PathBuf;

use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};
use rstar::RTree;
use types::*;

pub mod test_utils;

#[derive(Debug, Clone)]
pub enum Points {
    Vec(Vec<Point>),
    Indexed(rstar::RTree<Point>),
}

impl Points {
    /// Consume self and return a indexed version of the points.
    pub fn initialize_index(self) -> Self {
        if let Self::Vec(points) = self {
            Self::Indexed(RTree::bulk_load(points))
        } else {
            println!("This pointcould is already indexed.");
            self
        }
    }

    /// Get the amount of points in a pointcloud.
    pub fn count(&self) -> usize {
        match self {
            Points::Vec(points) => points.len(),
            Points::Indexed(rtree) => rtree.size(),
        }
    }

    /// Add a point to the pointcloud.
    pub fn add_point(&mut self, point: Point) {
        match self {
            Points::Vec(points) => points.push(point),
            Points::Indexed(rtree) => rtree.insert(point),
        };
    }

    /// Add multiple points to a pointcloud.
    pub fn add_points(&mut self, points: Vec<Point>) {
        let mut p2 = points;
        match self {
            Points::Vec(_points) => _points.append(&mut p2),
            Points::Indexed(rtree) => p2.into_iter().for_each(|x| rtree.insert(x)),
        };
    }
}

#[derive(Debug, Clone)]
pub struct Pointcloud {
    /// The offset of the pointcloud. This should be handled when the user needs to see the actual
    /// location of the point.
    pub offset: Vector3,
    /// The scale of the pointcloud. This should be handled when the user needs to see the actual
    /// location of the point.
    pub scale: Vector3,
    /// The boundary of the pointcloud in real measurements
    pub bounds: Bounds,
    /// The points.
    pub points: Points,
}

impl Pointcloud {
    /// Read a pointcloud from a las file.
    pub fn read_from_las(path: PathBuf) -> Self {
        let mut pcloud = las::Reader::from_path(path).unwrap();
        let header = pcloud.header().clone().into_raw().unwrap();

        let mut points_vector: Vec<Point> = pcloud
            .points()
            .flatten()
            .map(|x| Point {
                pos: Vector3 {
                    x: x.x,
                    y: x.y,
                    z: x.z,
                },
                intensity: x.intensity,
                color: x
                    .color
                    .map(|color| Color::new(color.red, color.green, color.blue)),
            })
            .collect();

        // Some las files store colors as u8. If they use u8 then translate it to u16
        let color_bounds = points_vector
            .par_iter()
            .filter_map(|x| x.color)
            .map(|x| (x.r.max(x.g.max(x.b)), x.r.min(x.g.min(x.b))))
            .reduce_with(|a, b| (a.0.max(b.0), a.1.min(b.1)));

        if let Some((max, min)) = color_bounds {
            if max == 0 && min == 0 {
            } else if max <= u8::MAX as u16 {
                points_vector.par_iter_mut().for_each(|x| {
                    x.color = x.color.map(|color| Color {
                        r: color.r.pow(2),
                        g: color.g.pow(2),
                        b: color.b.pow(2),
                    });
                });
            }
        }

        Self {
            offset: Vector3 {
                x: header.x_offset,
                y: header.y_offset,
                z: header.z_offset,
            },
            scale: Vector3 {
                x: header.x_scale_factor,
                y: header.y_scale_factor,
                z: header.z_scale_factor,
            },
            bounds: Bounds {
                min: Vector3 {
                    x: header.min_x,
                    y: header.min_y,
                    z: header.min_z,
                },
                max: Vector3 {
                    x: header.max_x,
                    y: header.max_y,
                    z: header.max_z,
                },
            },
            points: Points::Vec(points_vector),
        }
    }

    /// Check if a pointcloud is indexed.
    pub fn indexed(&self) -> bool {
        match &self.points {
            Points::Vec(_) => false,
            Points::Indexed(_) => true,
        }
    }

    /// Returns a indexed version of the pointcloud
    pub fn index(mut self) -> Self {
        self.points = self.points.initialize_index();
        self
    }
}
