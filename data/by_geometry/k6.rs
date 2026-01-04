use crate::core::geometry::Coordinates;

pub struct K6Geometry;

impl K6Geometry {
    pub const POINTS: [Coordinates; 6] = [
        Coordinates { x: 0.0, y: 1.0, z: None },
        Coordinates { x: 0.866, y: 0.5, z: None },
        Coordinates { x: 0.866, y: -0.5, z: None },
        Coordinates { x: 0.0, y: -1.0, z: None },
        Coordinates { x: -0.866, y: -0.5, z: None },
        Coordinates { x: -0.866, y: 0.5, z: None },
    ];
}
