use crate::core::geometry::Coordinates;

pub struct K9Geometry;

impl K9Geometry {
    pub const POINTS: [Coordinates; 9] = [
        Coordinates { x: 0.64278760968, y: 0.76604444311, z: None },
        Coordinates { x: 0.98480775301, y: 0.17364817767, z: None },
        Coordinates { x: 0.86602540378, y: -0.5, z: None },
        Coordinates { x: 0.34202014333, y: -0.93969262079, z: None },
        Coordinates { x: -0.34202014333, y: -0.93969262079, z: None },
        Coordinates { x: -0.86602540378, y: -0.5, z: None },
        Coordinates { x: -0.98480775301, y: 0.17364817767, z: None },
        Coordinates { x: -0.64278760968, y: 0.76604444311, z: None },
        Coordinates { x: 0.0, y: 1.0, z: None },
    ];
}
