use pointcloud::{Bounds, Point, Vector3};

#[test]
fn test_no_points() {
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

#[test]
fn test_one_points() {
    let p1 = Vector3 {
        x: 1.0,
        y: -2.0,
        z: 5.0,
    };

    let empty_points: Vec<Point> = vec![Point { pos: p1.clone() }];
    let bounds: Bounds = (&empty_points).into();

    assert_eq!(
        bounds,
        Bounds {
            max: p1.clone(),
            min: p1.clone(),
        }
    )
}

#[test]
fn test_many_points() {
    let points: Vec<Point> = vec![
        Point {
            pos: Vector3 {
                x: 1.0,
                y: -2.0,
                z: 5.0,
            },
        },
        Point {
            pos: Vector3 {
                x: 5.2,
                y: 22.0,
                z: 5.0,
            },
        },
        Point {
            pos: Vector3 {
                x: -21.0,
                y: 0.0,
                z: 0.0,
            },
        },
        Point {
            pos: Vector3 {
                x: 25.0,
                y: -4.0,
                z: 5.2,
            },
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
