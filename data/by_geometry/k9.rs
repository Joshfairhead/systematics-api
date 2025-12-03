use crate::core::geometry::Coordinates;

pub struct K9Geometry;

impl K9Geometry {
    pub const POINTS: [Coordinates; 9] = [
        Coordinates { x: 0.64278760968, y: 0.76604444311, z: None },    // 0: top-right
        Coordinates { x: 0.98480775301, y: 0.17364817767, z: None },    // 1: right
        Coordinates { x: 0.86602540378, y: -0.5, z: None },             // 2: bottom-right
        Coordinates { x: 0.34202014333, y: -0.93969262079, z: None },   // 3: bottom
        Coordinates { x: -0.34202014333, y: -0.93969262079, z: None },  // 4: bottom-left
        Coordinates { x: -0.86602540378, y: -0.5, z: None },            // 5: left
        Coordinates { x: -0.98480775301, y: 0.17364817767, z: None },   // 6: top-left
        Coordinates { x: -0.64278760968, y: 0.76604444311, z: None },   // 7: top-left-2
        Coordinates { x: 0.0, y: 1.0, z: None },                        // 8: top
    ];
    pub const LINES: [(Coordinates, Coordinates); 36] = [
        (Coordinates { x: 0.64278760968, y: 0.76604444311, z: None }, Coordinates { x: 0.98480775301, y: 0.17364817767, z: None }),
        (Coordinates { x: 0.64278760968, y: 0.76604444311, z: None }, Coordinates { x: 0.86602540378, y: -0.5, z: None }),
        (Coordinates { x: 0.64278760968, y: 0.76604444311, z: None }, Coordinates { x: 0.34202014333, y: -0.93969262079, z: None }),
        (Coordinates { x: 0.64278760968, y: 0.76604444311, z: None }, Coordinates { x: -0.34202014333, y: -0.93969262079, z: None }),
        (Coordinates { x: 0.64278760968, y: 0.76604444311, z: None }, Coordinates { x: -0.86602540378, y: -0.5, z: None }),
        (Coordinates { x: 0.64278760968, y: 0.76604444311, z: None }, Coordinates { x: -0.98480775301, y: 0.17364817767, z: None }),
        (Coordinates { x: 0.64278760968, y: 0.76604444311, z: None }, Coordinates { x: -0.64278760968, y: 0.76604444311, z: None }),
        (Coordinates { x: 0.64278760968, y: 0.76604444311, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: 0.98480775301, y: 0.17364817767, z: None }, Coordinates { x: 0.86602540378, y: -0.5, z: None }),
        (Coordinates { x: 0.98480775301, y: 0.17364817767, z: None }, Coordinates { x: 0.34202014333, y: -0.93969262079, z: None }),
        (Coordinates { x: 0.98480775301, y: 0.17364817767, z: None }, Coordinates { x: -0.34202014333, y: -0.93969262079, z: None }),
        (Coordinates { x: 0.98480775301, y: 0.17364817767, z: None }, Coordinates { x: -0.86602540378, y: -0.5, z: None }),
        (Coordinates { x: 0.98480775301, y: 0.17364817767, z: None }, Coordinates { x: -0.98480775301, y: 0.17364817767, z: None }),
        (Coordinates { x: 0.98480775301, y: 0.17364817767, z: None }, Coordinates { x: -0.64278760968, y: 0.76604444311, z: None }),
        (Coordinates { x: 0.98480775301, y: 0.17364817767, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: 0.86602540378, y: -0.5, z: None }, Coordinates { x: 0.34202014333, y: -0.93969262079, z: None }),
        (Coordinates { x: 0.86602540378, y: -0.5, z: None }, Coordinates { x: -0.34202014333, y: -0.93969262079, z: None }),
        (Coordinates { x: 0.86602540378, y: -0.5, z: None }, Coordinates { x: -0.86602540378, y: -0.5, z: None }),
        (Coordinates { x: 0.86602540378, y: -0.5, z: None }, Coordinates { x: -0.98480775301, y: 0.17364817767, z: None }),
        (Coordinates { x: 0.86602540378, y: -0.5, z: None }, Coordinates { x: -0.64278760968, y: 0.76604444311, z: None }),
        (Coordinates { x: 0.86602540378, y: -0.5, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: 0.34202014333, y: -0.93969262079, z: None }, Coordinates { x: -0.34202014333, y: -0.93969262079, z: None }),
        (Coordinates { x: 0.34202014333, y: -0.93969262079, z: None }, Coordinates { x: -0.86602540378, y: -0.5, z: None }),
        (Coordinates { x: 0.34202014333, y: -0.93969262079, z: None }, Coordinates { x: -0.98480775301, y: 0.17364817767, z: None }),
        (Coordinates { x: 0.34202014333, y: -0.93969262079, z: None }, Coordinates { x: -0.64278760968, y: 0.76604444311, z: None }),
        (Coordinates { x: 0.34202014333, y: -0.93969262079, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: -0.34202014333, y: -0.93969262079, z: None }, Coordinates { x: -0.86602540378, y: -0.5, z: None }),
        (Coordinates { x: -0.34202014333, y: -0.93969262079, z: None }, Coordinates { x: -0.98480775301, y: 0.17364817767, z: None }),
        (Coordinates { x: -0.34202014333, y: -0.93969262079, z: None }, Coordinates { x: -0.64278760968, y: 0.76604444311, z: None }),
        (Coordinates { x: -0.34202014333, y: -0.93969262079, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: -0.86602540378, y: -0.5, z: None }, Coordinates { x: -0.98480775301, y: 0.17364817767, z: None }),
        (Coordinates { x: -0.86602540378, y: -0.5, z: None }, Coordinates { x: -0.64278760968, y: 0.76604444311, z: None }),
        (Coordinates { x: -0.86602540378, y: -0.5, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: -0.98480775301, y: 0.17364817767, z: None }, Coordinates { x: -0.64278760968, y: 0.76604444311, z: None }),
        (Coordinates { x: -0.98480775301, y: 0.17364817767, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: -0.64278760968, y: 0.76604444311, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
    ];
} 