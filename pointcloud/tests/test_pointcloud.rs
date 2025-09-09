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

    #[test]
    fn add_point() {
        todo!()
    }

    #[test]
    fn remove_point() {
        todo!()
    }

    #[test]
    fn add_points() {
        todo!()
    }

    #[test]
    fn remove_points() {
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
