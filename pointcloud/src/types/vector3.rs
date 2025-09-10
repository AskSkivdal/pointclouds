use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    PartialOrd,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    /// Get the minimum value of all parts individualy
    pub fn min(&self, b: &Self) -> Self {
        Self {
            x: self.x.min(b.x),
            y: self.y.min(b.y),
            z: self.z.min(b.z),
        }
    }

    /// Get the maximum value of all parts individualy
    pub fn max(&self, b: &Self) -> Self {
        Self {
            x: self.x.max(b.x),
            y: self.y.max(b.y),
            z: self.z.max(b.z),
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl rstar::Point for Vector3 {
    type Scalar = f64;

    const DIMENSIONS: usize = 3;

    fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
        Self {
            x: generator(0),
            y: generator(1),
            z: generator(2),
        }
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Index out of bounds for Vector3: {}", index),
        }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds for Vector3: {}", index),
        }
    }
}
