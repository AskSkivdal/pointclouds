use pointcloud::types::{Bounds, Point, Vector3};

#[cfg(test)]
mod point {
    use super::*;
    /// Test creating a boundary fromm zero points.
    #[test]
    fn test_none() {
        let empty_points: Vec<Point> = Vec::new();
        let bounds: Bounds = (&empty_points).into();

        assert_eq!(
            bounds,
            Bounds {
                max: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0
                },
                min: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0
                },
            }
        )
    }

    /// Test creating a boundary from one point.
    #[test]
    fn test_one() {
        let p1 = Vector3 {
            x: 1.0,
            y: -2.0,
            z: 5.0,
        };

        let empty_points: Vec<Point> = vec![Point {
            pos: p1,
            intensity: 0,
            color: None,
        }];
        let bounds: Bounds = (&empty_points).into();

        assert_eq!(
            bounds,
            Bounds {
                max: p1,
                min: p1,
            }
        )
    }

    /// Test creating a boundary from many points.
    #[test]
    fn test_many() {
        let points: Vec<Point> = vec![
            Point {
                pos: Vector3 {
                    x: 1.0,
                    y: -2.0,
                    z: 5.0,
                },
                intensity: 0,
                color: None,
            },
            Point {
                pos: Vector3 {
                    x: 5.2,
                    y: 22.0,
                    z: 5.0,
                },
                intensity: 0,
                color: None,
            },
            Point {
                pos: Vector3 {
                    x: -21.0,
                    y: 0.0,
                    z: 0.0,
                },
                intensity: 0,
                color: None,
            },
            Point {
                pos: Vector3 {
                    x: 25.0,
                    y: -4.0,
                    z: 5.2,
                },
                intensity: 0,
                color: None,
            },
        ];
        let bounds: Bounds = (&points).into();

        assert_eq!(
            bounds,
            Bounds {
                max: Vector3 {
                    x: 25.0,
                    y: 22.0,
                    z: 5.2
                },
                min: Vector3 {
                    x: -21.0,
                    y: -4.0,
                    z: 0.0
                },
            }
        )
    }
}

#[cfg(test)]
mod vector {
    use super::*;
    /// Test creating a boundary from no vectors.
    #[test]
    fn test_none() {
        let empty_points: Vec<Vector3> = Vec::new();
        let bounds: Bounds = (&empty_points).into();

        assert_eq!(
            bounds,
            Bounds {
                max: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0
                },
                min: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0
                },
            }
        )
    }

    /// Test creating a boundary from one vector.
    #[test]
    fn test_one() {
        let p1 = Vector3 {
            x: 1.0,
            y: -2.0,
            z: 5.0,
        };

        let empty_points: Vec<Vector3> = vec![p1];
        let bounds: Bounds = (&empty_points).into();

        assert_eq!(
            bounds,
            Bounds {
                max: p1,
                min: p1,
            }
        )
    }

    /// Test creating a boundary from many vectors.
    #[test]
    fn test_many() {
        let points: Vec<Vector3> = vec![
            Vector3 {
                x: 1.0,
                y: -2.0,
                z: 5.0,
            },
            Vector3 {
                x: 5.2,
                y: 22.0,
                z: 5.0,
            },
            Vector3 {
                x: -21.0,
                y: 0.0,
                z: 0.0,
            },
            Vector3 {
                x: 25.0,
                y: -4.0,
                z: 5.2,
            },
        ];
        let bounds: Bounds = (&points).into();

        assert_eq!(
            bounds,
            Bounds {
                max: Vector3 {
                    x: 25.0,
                    y: 22.0,
                    z: 5.2
                },
                min: Vector3 {
                    x: -21.0,
                    y: -4.0,
                    z: 0.0
                },
            }
        )
    }
}
