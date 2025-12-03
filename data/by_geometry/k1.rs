use crate::core::geometry::Coordinates;

pub struct K1Geometry;

impl K1Geometry {
    pub const POINTS: [Coordinates; 1] = [
        Coordinates { x: 0.0, y: 0.0, z: None }, // Single point
    ];
    pub const LINES: [(Coordinates, Coordinates); 0] = [];
} 