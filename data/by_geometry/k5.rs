use crate::core::geometry::Coordinates;

pub struct K5Geometry;

impl K5Geometry {
    pub const POINTS: [Coordinates; 5] = [
        Coordinates { x: 1.0, y: 0.75, z: None },
        Coordinates { x: 0.0, y: 0.5, z: None },
        Coordinates { x: -0.75, y: 0.0, z: None },
        Coordinates { x: 0.0, y: -0.5, z: None },
        Coordinates { x: 1.0, y: -0.75, z: None },
    ];
}
