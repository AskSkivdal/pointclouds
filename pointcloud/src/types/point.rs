use rstar::AABB;

use crate::types::Vector3;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Point {
    pub pos: Vector3,
}

impl rstar::RTreeObject for Point {
    type Envelope = AABB<Vector3>;

    fn envelope(&self) -> Self::Envelope {
        self.pos.envelope()
    }
}
