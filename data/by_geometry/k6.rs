use crate::core::geometry::Coordinates;

pub struct K6Geometry;

impl K6Geometry {
    pub const POINTS: [Coordinates; 6] = [
        Coordinates { x: 0.0, y: 1.0, z: None },            // index 0: top point
        Coordinates { x: 0.866, y: 0.5, z: None },          // index 1: upper right
        Coordinates { x: 0.866, y: -0.5, z: None },         // index 2: lower right
        Coordinates { x: 0.0, y: -1.0, z: None },           // index 3: bottom point
        Coordinates { x: -0.866, y: -0.5, z: None },        // index 4: lower left
        Coordinates { x: -0.866, y: 0.5, z: None },         // index 5: upper left
    ];
    pub const LINES: [(Coordinates, Coordinates); 15] = [
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: 0.866, y: 0.5, z: None }),         // 0-1
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: 0.866, y: -0.5, z: None }),        // 0-2
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }),          // 0-3
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: -0.866, y: -0.5, z: None }),       // 0-4
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: -0.866, y: 0.5, z: None }),        // 0-5
        (Coordinates { x: 0.866, y: 0.5, z: None }, Coordinates { x: 0.866, y: -0.5, z: None }),      // 1-2
        (Coordinates { x: 0.866, y: 0.5, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }),        // 1-3
        (Coordinates { x: 0.866, y: 0.5, z: None }, Coordinates { x: -0.866, y: -0.5, z: None }),     // 1-4
        (Coordinates { x: 0.866, y: 0.5, z: None }, Coordinates { x: -0.866, y: 0.5, z: None }),      // 1-5
        (Coordinates { x: 0.866, y: -0.5, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }),       // 2-3
        (Coordinates { x: 0.866, y: -0.5, z: None }, Coordinates { x: -0.866, y: -0.5, z: None }),    // 2-4
        (Coordinates { x: 0.866, y: -0.5, z: None }, Coordinates { x: -0.866, y: 0.5, z: None }),     // 2-5
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: -0.866, y: -0.5, z: None }),      // 3-4
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: -0.866, y: 0.5, z: None }),       // 3-5
        (Coordinates { x: -0.866, y: -0.5, z: None }, Coordinates { x: -0.866, y: 0.5, z: None }),    // 4-5
    ];
} 