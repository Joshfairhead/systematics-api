use crate::core::geometry::Coordinates;

pub struct K5Geometry;

impl K5Geometry {
    pub const POINTS: [Coordinates; 5] = [
        Coordinates { x: 1.0, y: 0.75, z: None },    // 0: Purpose (right, top)
        Coordinates { x: 0.0, y: 0.5, z: None },     // 1: Higher Potential (center, upper)
        Coordinates { x: -0.75, y: 0.0, z: None },   // 2: Quintessence (left-center, middle)
        Coordinates { x: 0.0, y: -0.5, z: None },    // 3: Lower Potential (center, lower)
        Coordinates { x: 1.0, y: -0.75, z: None },   // 4: Source (right, bottom)
    ];
    pub const LINES: [(Coordinates, Coordinates); 10] = [
        (Coordinates { x: 1.0, y: 0.75, z: None }, Coordinates { x: 0.0, y: 0.5, z: None }), // Purpose-Higher Potential
        (Coordinates { x: 0.0, y: 0.5, z: None }, Coordinates { x: -0.75, y: 0.0, z: None }), // Higher Potential-Quintessence
        (Coordinates { x: -0.75, y: 0.0, z: None }, Coordinates { x: 0.0, y: -0.5, z: None }), // Quintessence-Lower Potential
        (Coordinates { x: 0.0, y: -0.5, z: None }, Coordinates { x: 1.0, y: -0.75, z: None }), // Lower Potential-Source
        (Coordinates { x: 1.0, y: -0.75, z: None }, Coordinates { x: 1.0, y: 0.75, z: None }), // Source-Purpose
        (Coordinates { x: 1.0, y: 0.75, z: None }, Coordinates { x: -0.75, y: 0.0, z: None }), // Purpose-Quintessence
        (Coordinates { x: 0.0, y: 0.5, z: None }, Coordinates { x: 0.0, y: -0.5, z: None }), // Higher Potential-Lower Potential
        (Coordinates { x: -0.75, y: 0.0, z: None }, Coordinates { x: 1.0, y: -0.75, z: None }), // Quintessence-Source
        (Coordinates { x: 0.0, y: -0.5, z: None }, Coordinates { x: 1.0, y: 0.75, z: None }), // Lower Potential-Purpose
        (Coordinates { x: 1.0, y: -0.75, z: None }, Coordinates { x: 0.0, y: 0.5, z: None }), // Source-Higher Potential
    ];
} 