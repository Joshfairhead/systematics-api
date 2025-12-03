use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}

impl Coordinates {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y, z: None }
    }

    pub fn new_3d(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z: Some(z) }
    }
}

pub type Point = Coordinates;
pub type Line = (Coordinates, Coordinates);
