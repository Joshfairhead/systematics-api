use crate::core::state_manager::Coordinates;

pub struct K3Geometry;

impl K3Geometry {
    pub const POINTS: [Coordinates; 3] = [
        Coordinates { x: 0.0, y: 1.0, z: None },   // Will (top left)
        Coordinates { x: 0.0, y: -1.0, z: None },  // Function (bottom left)
        Coordinates { x: 1.0, y: 0.0, z: None },   // Being (right, midpoint vertically)
    ];
    pub const LINES: [(Coordinates, Coordinates); 3] = [
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }),  // Will-Function (vertical left)
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: 1.0, y: 0.0, z: None }),  // Function-Being (bottom right)
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),  // Being-Will (top right)
    ];
} 