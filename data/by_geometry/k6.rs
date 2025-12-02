use crate::core::state_manager::Coordinates;

pub struct K6Geometry;

impl K6Geometry {
    pub const POINTS: [Coordinates; 6] = [
        Coordinates { x: -0.5, y: 0.86602540378, z: None }, // index 0: top left
        Coordinates { x: 0.0, y: 1.0, z: None },            // index 1: top tip
        Coordinates { x: 0.5, y: 0.86602540378, z: None },  // index 2: top right
        Coordinates { x: 1.0, y: 0.0, z: None },            // index 3: bottom right
        Coordinates { x: 0.0, y: -1.0, z: None },           // index 4: bottom
        Coordinates { x: -1.0, y: 0.0, z: None },           // index 5: bottom left
    ];
    pub const LINES: [(Coordinates, Coordinates); 15] = [
        (Coordinates { x: -0.5, y: 0.86602540378, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }), (Coordinates { x: -0.5, y: 0.86602540378, z: None }, Coordinates { x: 0.5, y: 0.86602540378, z: None }), (Coordinates { x: -0.5, y: 0.86602540378, z: None }, Coordinates { x: 1.0, y: 0.0, z: None }), (Coordinates { x: -0.5, y: 0.86602540378, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }), (Coordinates { x: -0.5, y: 0.86602540378, z: None }, Coordinates { x: -1.0, y: 0.0, z: None }),
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: 0.5, y: 0.86602540378, z: None }), (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: 1.0, y: 0.0, z: None }), (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }), (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: -1.0, y: 0.0, z: None }),
        (Coordinates { x: 0.5, y: 0.86602540378, z: None }, Coordinates { x: 1.0, y: 0.0, z: None }), (Coordinates { x: 0.5, y: 0.86602540378, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }), (Coordinates { x: 0.5, y: 0.86602540378, z: None }, Coordinates { x: -1.0, y: 0.0, z: None }),
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }), (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: -1.0, y: 0.0, z: None }),
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: -1.0, y: 0.0, z: None }),
    ];
} 