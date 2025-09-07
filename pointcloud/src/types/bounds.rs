use crate::types::{Point, Vector3};

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Bounds {
    pub min: Vector3,
    pub max: Vector3,
}

impl Bounds {
    /// Adjust the bounds to contain the Vector3. If the Vector3 is already within the bounds then
    /// it will not change the bounds
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

impl Into<Bounds> for &Vec<Vector3> {
    fn into(self) -> Bounds {
        let mut point_iterator = self.iter();

        let mut bounds: Bounds = if let Some(point) = point_iterator.next() {
            Bounds {
                max: *point,
                min: *point,
            }
        } else {
            Bounds::default()
        };

        for point in point_iterator {
            bounds.grow_with_vector(&point);
        }

        bounds
    }
}
