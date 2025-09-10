#[cfg(test)]
mod stats {
    use pointcloud::test_utils::mock_pointcloud;

    #[test]
    fn is_indexed() {
        let mut pc = mock_pointcloud();
        assert!(!pc.indexed());
        pc.points = pc.points.initialize_index();
        assert!(pc.indexed())
    }

    #[test]
    fn point_count() {
        let pc = mock_pointcloud();
        assert_eq!(pc.points.count(), 4);
    }

    #[test]
    fn get_bounds() {
        todo!()
    }

    #[test]
    fn get_bounds_miss() {
        todo!()
    }
}

#[cfg(test)]
mod modify {
    use pointcloud::{
        test_utils::mock_pointcloud,
        types::{Point, Vector3},
    };

    #[test]
    fn add_point() {
        let mut pc = mock_pointcloud();
        pc.points.add_point(Point {
            pos: Vector3::new(1.0, 0.0, 0.0),
            intensity: 234,
            color: None,
        });
        assert_eq!(pc.points.count(), 5);
    }

    #[test]
    fn add_point_indexed() {
        let mut pc = mock_pointcloud();
        pc.points = pc.points.initialize_index();
        pc.points.add_point(Point {
            pos: Vector3::new(1.0, 0.0, 0.0),
            intensity: 234,
            color: None,
        });
        assert_eq!(pc.points.count(), 5);
    }

    #[test]
    fn add_points() {
        let mut pc = mock_pointcloud();
        pc.points.add_points(vec![
            Point {
                pos: Vector3::new(1.0, 0.0, 0.0),
                intensity: 234,
                color: None,
            },
            Point {
                pos: Vector3::new(1.0, 2.4, -2.1),
                intensity: 543,
                color: None,
            },
        ]);
        assert_eq!(pc.points.count(), 6);
    }

    #[test]
    fn add_points_indexed() {
        let mut pc = mock_pointcloud();
        pc.points = pc.points.initialize_index();
        pc.points.add_points(vec![
            Point {
                pos: Vector3::new(1.0, 0.0, 0.0),
                intensity: 234,
                color: None,
            },
            Point {
                pos: Vector3::new(1.0, 2.4, -2.1),
                intensity: 543,
                color: None,
            },
        ]);
        assert_eq!(pc.points.count(), 6);
    }

    #[test]
    fn remove_point() {
        todo!()
    }

    #[test]
    fn remove_point_indexed() {
        todo!()
    }

    #[test]
    fn remove_points() {
        todo!()
    }

    #[test]
    fn remove_points_indexed() {
        todo!()
    }
}

#[cfg(test)]
mod query {
    #[test]
    fn get_by_distance_miss() {
        todo!()
    }

    #[test]
    fn get_by_distance_hit() {
        todo!()
    }

    #[test]
    fn get_in_bbox_miss() {
        todo!()
    }

    #[test]
    fn get_in_bbox_hit() {
        todo!()
    }

    #[test]
    fn get_closest_miss() {
        todo!()
    }

    #[test]
    fn get_closest_hit() {
        todo!()
    }
}
