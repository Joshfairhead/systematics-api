use crate::core::geometry::Coordinates;

pub struct K4Geometry;

impl K4Geometry {
    pub const POINTS: [Coordinates; 4] = [
        Coordinates { x: 0.0, y: 1.0, z: None },
        Coordinates { x: 1.0, y: 0.0, z: None },
        Coordinates { x: -1.0, y: 0.0, z: None },
        Coordinates { x: 0.0, y: -1.0, z: None },
    ];
}
