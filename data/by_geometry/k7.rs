use crate::core::geometry::Coordinates;

pub struct K7Geometry;

impl K7Geometry {
    pub const POINTS: [Coordinates; 7] = [
        Coordinates { x: 0.0, y: 1.0, z: None },
        Coordinates { x: 0.781831, y: 0.623489, z: None },
        Coordinates { x: 0.974370, y: -0.222521, z: None },
        Coordinates { x: 0.433884, y: -0.900969, z: None },
        Coordinates { x: -0.433884, y: -0.900969, z: None },
        Coordinates { x: -0.974370, y: -0.222521, z: None },
        Coordinates { x: -0.781831, y: 0.623489, z: None },
    ];
}
