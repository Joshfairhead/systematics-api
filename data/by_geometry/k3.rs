use crate::core::geometry::Coordinates;

pub struct K3Geometry;

impl K3Geometry {
    pub const POINTS: [Coordinates; 3] = [
        Coordinates { x: 0.0, y: 1.0, z: None },
        Coordinates { x: 0.0, y: -1.0, z: None },
        Coordinates { x: 1.0, y: 0.0, z: None },
    ];
}
