pub mod types;
use std::path::PathBuf;

use rstar::RTree;
use types::*;

#[derive(Debug, Clone)]
pub enum Points {
    Vec(Vec<Point>),
    Indexed(rstar::RTree<Point>),
}

impl Points {
    pub fn initialize_index(self) -> Self {
        if let Self::Vec(points) = self {
            Self::Indexed(RTree::bulk_load(points))
        } else {
            println!("This pointcould is already indexed.");
            self
        }
    }

    pub fn count(&self) -> usize {
        match self {
            Points::Vec(points) => points.len(),
            Points::Indexed(rtree) => rtree.size(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pointcloud {
    pub offset: Vector3,
    pub scale: Vector3,
    pub bounds: Bounds,
    pub points: Points,
}

impl Pointcloud {
    pub fn read_from_las(path: PathBuf) -> Self {
        let mut pcloud = las::Reader::from_path(path).unwrap();
        let header = pcloud.header().clone().into_raw().unwrap();

        let points_vector: Vec<Point> = pcloud
            .points()
            .flatten()
            .map(|x| Point {
                pos: Vector3 {
                    x: x.x,
                    y: x.y,
                    z: x.z,
                },
            })
            .collect();

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
}
