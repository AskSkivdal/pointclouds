#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn min(&self, b: &Self) -> Self {
        Self {
            x: self.x.min(b.x),
            y: self.y.min(b.y),
            z: self.z.min(b.z),
        }
    }

    pub fn max(&self, b: &Self) -> Self {
        Self {
            x: self.x.max(b.x),
            y: self.y.max(b.y),
            z: self.z.max(b.z),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Point {
    pub pos: Vector3,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Bounds {
    pub min: Vector3,
    pub max: Vector3,
}

impl Bounds {
    pub fn grow_with_vector(&mut self, point: &Vector3) {
        self.max = self.max.max(point);
        self.min = self.min.min(point);
    }
}

impl Into<Bounds> for &Vec<Point> {
    fn into(self) -> Bounds {
        let mut point_iterator = self.iter();

        let mut bounds: Bounds = if let Some(point) = point_iterator.next() {
            Bounds {
                max: point.pos,
                min: point.pos,
            }
        } else {
            Bounds::default()
        };

        for point in point_iterator {
            bounds.grow_with_vector(&point.pos);
        }

        bounds
    }
}

#[derive(Debug, Clone)]
pub struct Pointcloud {
    pub offset: Vector3,
    pub scale: Vector3,
    pub bounds: Bounds,
    pub points: Vec<Point>,
}
