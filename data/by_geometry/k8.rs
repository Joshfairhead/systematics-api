use crate::core::geometry::Coordinates;

pub struct K8Geometry;

impl K8Geometry {
    pub const POINTS: [Coordinates; 8] = [
        Coordinates { x: 1.0, y: 0.0, z: None },                        // 0: Smallest Significant Holon (right, middle)
        Coordinates { x: 0.70710678118, y: -0.70710678118, z: None },   // 1: Critical Functions (lower right)
        Coordinates { x: 0.0, y: -1.0, z: None },                       // 2: Supportive Platform (bottom)
        Coordinates { x: -0.70710678118, y: -0.70710678118, z: None },  // 3: Necessary Resourcing (lower left)
        Coordinates { x: -1.0, y: 0.0, z: None },                       // 4: Integrative Totality (left, middle)
        Coordinates { x: -0.70710678118, y: 0.70710678118, z: None },   // 5: Inherent Values (upper left)
        Coordinates { x: 0.0, y: 1.0, z: None },                        // 6: Intrinsic Nature (top)
        Coordinates { x: 0.70710678118, y: 0.70710678118, z: None },    // 7: Organisational Modes (upper right)
    ];
    pub const LINES: [(Coordinates, Coordinates); 28] = [
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: 0.70710678118, y: -0.70710678118, z: None }),
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }),
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: -0.70710678118, y: -0.70710678118, z: None }),
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: -1.0, y: 0.0, z: None }),
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: -0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: 0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: 0.70710678118, y: -0.70710678118, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }),
        (Coordinates { x: 0.70710678118, y: -0.70710678118, z: None }, Coordinates { x: -0.70710678118, y: -0.70710678118, z: None }),
        (Coordinates { x: 0.70710678118, y: -0.70710678118, z: None }, Coordinates { x: -1.0, y: 0.0, z: None }),
        (Coordinates { x: 0.70710678118, y: -0.70710678118, z: None }, Coordinates { x: -0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: 0.70710678118, y: -0.70710678118, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: 0.70710678118, y: -0.70710678118, z: None }, Coordinates { x: 0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: -0.70710678118, y: -0.70710678118, z: None }),
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: -1.0, y: 0.0, z: None }),
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: -0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: 0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: -0.70710678118, y: -0.70710678118, z: None }, Coordinates { x: -1.0, y: 0.0, z: None }),
        (Coordinates { x: -0.70710678118, y: -0.70710678118, z: None }, Coordinates { x: -0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: -0.70710678118, y: -0.70710678118, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: -0.70710678118, y: -0.70710678118, z: None }, Coordinates { x: 0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: -1.0, y: 0.0, z: None }, Coordinates { x: -0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: -1.0, y: 0.0, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: -1.0, y: 0.0, z: None }, Coordinates { x: 0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: -0.70710678118, y: 0.70710678118, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: -0.70710678118, y: 0.70710678118, z: None }, Coordinates { x: 0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: 0.70710678118, y: 0.70710678118, z: None }),
    ];
} 