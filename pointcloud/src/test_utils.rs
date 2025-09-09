use crate::{
    Pointcloud,
    types::{Bounds, Point, Vector3},
};

pub fn mock_pointcloud() -> Pointcloud {
    Pointcloud {
        offset: Vector3::new(0.0, 0.0, 0.0),
        scale: Vector3::new(1.0, 1.0, 1.0),
        bounds: Bounds::new(Vector3::new(-1.0, -1.0, -1.0), Vector3::new(1.0, 1.0, 1.0)),
        points: crate::Points::Vec(vec![
            Point {
                pos: Vector3::new(1.0, 1.0, 1.0),
                intensity: 23,
                color: None,
            },
            Point {
                pos: Vector3::new(0.0, -1.0, 0.2),
                intensity: 23,
                color: None,
            },
            Point {
                pos: Vector3::new(0.6, 0.5, -0.2),
                intensity: 23,
                color: None,
            },
            Point {
                pos: Vector3::new(1.0, 1.0, 1.0),
                intensity: 23,
                color: None,
            },
        ]),
    }
}
