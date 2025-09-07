pub mod types;
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
}

#[derive(Debug, Clone)]
pub struct Pointcloud {
    pub offset: Vector3,
    pub scale: Vector3,
    pub bounds: Bounds,
    pub points: Points,
}
