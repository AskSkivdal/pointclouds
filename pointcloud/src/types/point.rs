use rstar::AABB;

use crate::types::Vector3;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Color {
    pub r: u16,
    pub g: u16,
    pub b: u16,
}

impl Color {
    pub fn new(r: u16, g: u16, b: u16) -> Self {
        Self { r, g, b }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Point {
    pub pos: Vector3,
    pub intensity: u16,
    pub color: Option<Color>,
}

impl rstar::RTreeObject for Point {
    type Envelope = AABB<Vector3>;

    fn envelope(&self) -> Self::Envelope {
        self.pos.envelope()
    }
}
