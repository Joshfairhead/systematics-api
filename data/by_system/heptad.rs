use crate::core::state_manager::{Coordinates, Index};

pub struct HeptadSystem;

impl HeptadSystem {
    // Metadata
    pub const SYSTEM_NAME: &'static str = "Heptad";
    pub const COHERENCE_ATTRIBUTE: &'static str = "Generation";
    pub const TERM_DESIGNATION: &'static str = "States";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Intervals";
    pub const SOURCE: &'static str = "Elementary Systematics";

    // Vocabulary (from by_vocabulary/k7.rs)
    pub const TERM_CHARACTERS: [&'static str; 7] = ["Insight", "Research", "Design", "Synthesis", "Application", "Delivery", "Value"];
    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 21] = [
        ("Needs Research", "Insight", "Research"),
        ("Needs Research", "Insight", "Design"),
        ("Needs Research", "Insight", "Synthesis"),
        ("Needs Research", "Insight", "Application"),
        ("Needs Research", "Insight", "Delivery"),
        ("Needs Research", "Insight", "Value"),
        ("Needs Research", "Research", "Design"),
        ("Needs Research", "Research", "Synthesis"),
        ("Needs Research", "Research", "Application"),
        ("Needs Research", "Research", "Delivery"),
        ("Needs Research", "Research", "Value"),
        ("Needs Research", "Design", "Synthesis"),
        ("Needs Research", "Design", "Application"),
        ("Needs Research", "Design", "Delivery"),
        ("Needs Research", "Design", "Value"),
        ("Needs Research", "Synthesis", "Application"),
        ("Needs Research", "Synthesis", "Delivery"),
        ("Needs Research", "Synthesis", "Value"),
        ("Needs Research", "Application", "Delivery"),
        ("Needs Research", "Application", "Value"),
        ("Needs Research", "Delivery", "Value"),
    ];

    // Topology (from by_topology/k7.rs)
    pub const INDEX: [Index; 7] = [0, 1, 2, 3, 4, 5, 6];
    pub const INDICES: [(Index, Index); 21] = [
        (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6),
        (1, 2), (1, 3), (1, 4), (1, 5), (1, 6),
        (2, 3), (2, 4), (2, 5), (2, 6),
        (3, 4), (3, 5), (3, 6),
        (4, 5), (4, 6),
        (5, 6),
    ];

    // Geometry (from by_geometry/k7.rs)
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
