use crate::core::geometry::Coordinates;

pub struct K2Geometry;

impl K2Geometry {
    pub const POINTS: [Coordinates; 2] = [
        Coordinates { x: -1.0, y: 0.0, z: None }, // Left
        Coordinates { x: 1.0, y: 0.0, z: None },  // Right
    ];
    pub const LINES: [(Coordinates, Coordinates); 1] = [
        (Coordinates { x: -1.0, y: 0.0, z: None }, Coordinates { x: 1.0, y: 0.0, z: None }), // Single edge
    ];
} 