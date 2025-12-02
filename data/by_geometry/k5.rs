use crate::core::state_manager::Coordinates;

pub struct K5Geometry;

impl K5Geometry {
    pub const POINTS: [Coordinates; 5] = [
        Coordinates { x: 1.0, y: 1.0, z: None },     // 0: Purpose (upper right)
        Coordinates { x: 0.5, y: 0.5, z: None },     // 1: Higher Potential (middle, above quintessence, below purpose)
        Coordinates { x: -1.0, y: 0.0, z: None },    // 2: Quintessence (left, vertical midpoint)
        Coordinates { x: 0.5, y: -0.5, z: None },    // 3: Lower Potential (middle, below quintessence, above source)
        Coordinates { x: 1.0, y: -1.0, z: None },    // 4: Source (lower right)
    ];
    pub const LINES: [(Coordinates, Coordinates); 10] = [
        (Coordinates { x: 1.0, y: 1.0, z: None }, Coordinates { x: 0.5, y: 0.5, z: None }), // Purpose-Higher Potential
        (Coordinates { x: 0.5, y: 0.5, z: None }, Coordinates { x: -1.0, y: 0.0, z: None }), // Higher Potential-Quintessence
        (Coordinates { x: -1.0, y: 0.0, z: None }, Coordinates { x: 0.5, y: -0.5, z: None }), // Quintessence-Lower Potential
        (Coordinates { x: 0.5, y: -0.5, z: None }, Coordinates { x: 1.0, y: -1.0, z: None }), // Lower Potential-Source
        (Coordinates { x: 1.0, y: -1.0, z: None }, Coordinates { x: 1.0, y: 1.0, z: None }), // Source-Purpose
        (Coordinates { x: 1.0, y: 1.0, z: None }, Coordinates { x: -1.0, y: 0.0, z: None }), // Purpose-Quintessence
        (Coordinates { x: 0.5, y: 0.5, z: None }, Coordinates { x: 0.5, y: -0.5, z: None }), // Higher Potential-Lower Potential
        (Coordinates { x: -1.0, y: 0.0, z: None }, Coordinates { x: 1.0, y: -1.0, z: None }), // Quintessence-Source
        (Coordinates { x: 0.5, y: -0.5, z: None }, Coordinates { x: 1.0, y: 1.0, z: None }), // Lower Potential-Purpose
        (Coordinates { x: 1.0, y: -1.0, z: None }, Coordinates { x: 0.5, y: 0.5, z: None }), // Source-Higher Potential
    ];
} 