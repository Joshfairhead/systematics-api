use crate::core::geometry::Coordinates;

pub struct K8Geometry;

impl K8Geometry {
    pub const POINTS: [Coordinates; 8] = [
        Coordinates { x: 1.0, y: 0.0, z: None },
        Coordinates { x: 0.70710678118, y: -0.70710678118, z: None },
        Coordinates { x: 0.0, y: -1.0, z: None },
        Coordinates { x: -0.70710678118, y: -0.70710678118, z: None },
        Coordinates { x: -1.0, y: 0.0, z: None },
        Coordinates { x: -0.70710678118, y: 0.70710678118, z: None },
        Coordinates { x: 0.0, y: 1.0, z: None },
        Coordinates { x: 0.70710678118, y: 0.70710678118, z: None },
    ];
}
