use crate::core::state_manager::Coordinates;

pub struct K7Geometry;

impl K7Geometry {
    pub const POINTS: [Coordinates; 7] = [
        Coordinates { x: 0.0, y: 1.0, z: None },             // 0: Insight (top center)
        Coordinates { x: 0.781831, y: 0.623489, z: None },   // 1: Research
        Coordinates { x: 0.974370, y: -0.222521, z: None },  // 2: Design
        Coordinates { x: 0.433884, y: -0.900969, z: None },  // 3: Synthesis
        Coordinates { x: -0.433884, y: -0.900969, z: None }, // 4: Application
        Coordinates { x: -0.974370, y: -0.222521, z: None }, // 5: Delivery
        Coordinates { x: -0.781831, y: 0.623489, z: None },  // 6: Value
    ];
    pub const LINES: [(Coordinates, Coordinates); 21] = [
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: 0.781831, y: 0.623489, z: None }),
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: 0.974370, y: -0.222521, z: None }),
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: 0.433884, y: -0.900969, z: None }),
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: -0.433884, y: -0.900969, z: None }),
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: -0.974370, y: -0.222521, z: None }),
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: -0.781831, y: 0.623489, z: None }),
        (Coordinates { x: 0.781831, y: 0.623489, z: None }, Coordinates { x: 0.974370, y: -0.222521, z: None }),
        (Coordinates { x: 0.781831, y: 0.623489, z: None }, Coordinates { x: 0.433884, y: -0.900969, z: None }),
        (Coordinates { x: 0.781831, y: 0.623489, z: None }, Coordinates { x: -0.433884, y: -0.900969, z: None }),
        (Coordinates { x: 0.781831, y: 0.623489, z: None }, Coordinates { x: -0.974370, y: -0.222521, z: None }),
        (Coordinates { x: 0.781831, y: 0.623489, z: None }, Coordinates { x: -0.781831, y: 0.623489, z: None }),
        (Coordinates { x: 0.974370, y: -0.222521, z: None }, Coordinates { x: 0.433884, y: -0.900969, z: None }),
        (Coordinates { x: 0.974370, y: -0.222521, z: None }, Coordinates { x: -0.433884, y: -0.900969, z: None }),
        (Coordinates { x: 0.974370, y: -0.222521, z: None }, Coordinates { x: -0.974370, y: -0.222521, z: None }),
        (Coordinates { x: 0.974370, y: -0.222521, z: None }, Coordinates { x: -0.781831, y: 0.623489, z: None }),
        (Coordinates { x: 0.433884, y: -0.900969, z: None }, Coordinates { x: -0.433884, y: -0.900969, z: None }),
        (Coordinates { x: 0.433884, y: -0.900969, z: None }, Coordinates { x: -0.974370, y: -0.222521, z: None }),
        (Coordinates { x: 0.433884, y: -0.900969, z: None }, Coordinates { x: -0.781831, y: 0.623489, z: None }),
        (Coordinates { x: -0.433884, y: -0.900969, z: None }, Coordinates { x: -0.974370, y: -0.222521, z: None }),
        (Coordinates { x: -0.433884, y: -0.900969, z: None }, Coordinates { x: -0.781831, y: 0.623489, z: None }),
        (Coordinates { x: -0.974370, y: -0.222521, z: None }, Coordinates { x: -0.781831, y: 0.623489, z: None }),
    ];
} 