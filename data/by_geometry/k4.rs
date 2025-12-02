use crate::core::state_manager::Coordinates;

pub struct K4Geometry;

impl K4Geometry {
    pub const POINTS: [Coordinates; 4] = [
        Coordinates { x: 0.0, y: 1.0, z: None },   // ideal
        Coordinates { x: 1.0, y: 0.0, z: None },   // directive
        Coordinates { x: -1.0, y: 0.0, z: None },  // instrumental
        Coordinates { x: 0.0, y: -1.0, z: None },  // ground
    ];
    pub const LINES: [(Coordinates, Coordinates); 6] = [
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: 1.0, y: 0.0, z: None }),  // ideal-directive
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: -1.0, y: 0.0, z: None }),  // ideal-instrumental
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }),  // ideal-ground
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: -1.0, y: 0.0, z: None }),  // directive-instrumental
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }),  // directive-ground
        (Coordinates { x: -1.0, y: 0.0, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }),  // instrumental-ground
    ];
} 