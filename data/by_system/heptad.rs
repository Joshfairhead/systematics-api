use crate::core::topology::{Node, Edge};
use crate::core::geometry::Coordinates;

pub struct HeptadSystem;

impl HeptadSystem {
    // Metadata
    pub const SYSTEM_NAME: &'static str = "Heptad";
    pub const COHERENCE_ATTRIBUTE: &'static str = "Generation";
    pub const TERM_DESIGNATION: &'static str = "States";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Intervals";
    pub const SOURCE: &'static str = "Elementary Systematics";

    // Vocabulary (from by_vocabulary/k7.rs)
    // Semantic ordering: Research (1), Design (2), Synthesis (3), Application (4), Delivery (5), Value (6), Insight (7) when displayed with one-based indexing
    // Rotated clockwise by one position from original ordering
    pub const TERM_CHARACTERS: [&'static str; 7] = ["Research", "Design", "Synthesis", "Application", "Delivery", "Value", "Insight"];
    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 21] = [
        ("Needs Research", "Research", "Design"),
        ("Needs Research", "Research", "Synthesis"),
        ("Needs Research", "Research", "Application"),
        ("Needs Research", "Research", "Delivery"),
        ("Needs Research", "Research", "Value"),
        ("Needs Research", "Research", "Insight"),
        ("Needs Research", "Design", "Synthesis"),
        ("Needs Research", "Design", "Application"),
        ("Needs Research", "Design", "Delivery"),
        ("Needs Research", "Design", "Value"),
        ("Needs Research", "Design", "Insight"),
        ("Needs Research", "Synthesis", "Application"),
        ("Needs Research", "Synthesis", "Delivery"),
        ("Needs Research", "Synthesis", "Value"),
        ("Needs Research", "Synthesis", "Insight"),
        ("Needs Research", "Application", "Delivery"),
        ("Needs Research", "Application", "Value"),
        ("Needs Research", "Application", "Insight"),
        ("Needs Research", "Delivery", "Value"),
        ("Needs Research", "Delivery", "Insight"),
        ("Needs Research", "Value", "Insight"),
    ];

    // Topology (from by_topology/k7.rs)
    pub const NODES: [Node; 7] = [0, 1, 2, 3, 4, 5, 6];
    pub const EDGES: [Edge; 21] = [
        (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6),
        (1, 2), (1, 3), (1, 4), (1, 5), (1, 6),
        (2, 3), (2, 4), (2, 5), (2, 6),
        (3, 4), (3, 5), (3, 6),
        (4, 5), (4, 6),
        (5, 6),
    ];

    // Geometry (from by_geometry/k7.rs)
    // Coordinates rotated clockwise by one position to match semantic vocabulary ordering
    pub const POINTS: [Coordinates; 7] = [
        Coordinates { x: 0.781831, y: 0.623489, z: None },   // 0: Research (was index 1)
        Coordinates { x: 0.974370, y: -0.222521, z: None },  // 1: Design (was index 2)
        Coordinates { x: 0.433884, y: -0.900969, z: None },  // 2: Synthesis (was index 3)
        Coordinates { x: -0.433884, y: -0.900969, z: None }, // 3: Application (was index 4)
        Coordinates { x: -0.974370, y: -0.222521, z: None }, // 4: Delivery (was index 5)
        Coordinates { x: -0.781831, y: 0.623489, z: None },  // 5: Value (was index 6)
        Coordinates { x: 0.0, y: 1.0, z: None },             // 6: Insight (was index 0, top center)
    ];
    pub const LINES: [(Coordinates, Coordinates); 21] = [
        // Edge 0-1: Research-Design
        (Coordinates { x: 0.781831, y: 0.623489, z: None }, Coordinates { x: 0.974370, y: -0.222521, z: None }),
        // Edge 0-2: Research-Synthesis
        (Coordinates { x: 0.781831, y: 0.623489, z: None }, Coordinates { x: 0.433884, y: -0.900969, z: None }),
        // Edge 0-3: Research-Application
        (Coordinates { x: 0.781831, y: 0.623489, z: None }, Coordinates { x: -0.433884, y: -0.900969, z: None }),
        // Edge 0-4: Research-Delivery
        (Coordinates { x: 0.781831, y: 0.623489, z: None }, Coordinates { x: -0.974370, y: -0.222521, z: None }),
        // Edge 0-5: Research-Value
        (Coordinates { x: 0.781831, y: 0.623489, z: None }, Coordinates { x: -0.781831, y: 0.623489, z: None }),
        // Edge 0-6: Research-Insight
        (Coordinates { x: 0.781831, y: 0.623489, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        // Edge 1-2: Design-Synthesis
        (Coordinates { x: 0.974370, y: -0.222521, z: None }, Coordinates { x: 0.433884, y: -0.900969, z: None }),
        // Edge 1-3: Design-Application
        (Coordinates { x: 0.974370, y: -0.222521, z: None }, Coordinates { x: -0.433884, y: -0.900969, z: None }),
        // Edge 1-4: Design-Delivery
        (Coordinates { x: 0.974370, y: -0.222521, z: None }, Coordinates { x: -0.974370, y: -0.222521, z: None }),
        // Edge 1-5: Design-Value
        (Coordinates { x: 0.974370, y: -0.222521, z: None }, Coordinates { x: -0.781831, y: 0.623489, z: None }),
        // Edge 1-6: Design-Insight
        (Coordinates { x: 0.974370, y: -0.222521, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        // Edge 2-3: Synthesis-Application
        (Coordinates { x: 0.433884, y: -0.900969, z: None }, Coordinates { x: -0.433884, y: -0.900969, z: None }),
        // Edge 2-4: Synthesis-Delivery
        (Coordinates { x: 0.433884, y: -0.900969, z: None }, Coordinates { x: -0.974370, y: -0.222521, z: None }),
        // Edge 2-5: Synthesis-Value
        (Coordinates { x: 0.433884, y: -0.900969, z: None }, Coordinates { x: -0.781831, y: 0.623489, z: None }),
        // Edge 2-6: Synthesis-Insight
        (Coordinates { x: 0.433884, y: -0.900969, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        // Edge 3-4: Application-Delivery
        (Coordinates { x: -0.433884, y: -0.900969, z: None }, Coordinates { x: -0.974370, y: -0.222521, z: None }),
        // Edge 3-5: Application-Value
        (Coordinates { x: -0.433884, y: -0.900969, z: None }, Coordinates { x: -0.781831, y: 0.623489, z: None }),
        // Edge 3-6: Application-Insight
        (Coordinates { x: -0.433884, y: -0.900969, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        // Edge 4-5: Delivery-Value
        (Coordinates { x: -0.974370, y: -0.222521, z: None }, Coordinates { x: -0.781831, y: 0.623489, z: None }),
        // Edge 4-6: Delivery-Insight
        (Coordinates { x: -0.974370, y: -0.222521, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        // Edge 5-6: Value-Insight
        (Coordinates { x: -0.781831, y: 0.623489, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
    ];
}
