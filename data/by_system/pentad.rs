use crate::core::topology::{Node, Edge};
use crate::core::geometry::Coordinates;

pub struct PentadSystem;

impl PentadSystem {
    // Metadata
    pub const SYSTEM_NAME: &'static str = "Pentad";
    pub const COHERENCE_ATTRIBUTE: &'static str = "Significance and Potential";
    pub const TERM_DESIGNATION: &'static str = "Limits";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Mutualities";
    pub const SOURCE: &'static str = "Elementary Systematics";

    // Vocabulary (from by_vocabulary/k5.rs)
    pub const TERM_CHARACTERS: [&'static str; 5] = ["Purpose", "Higher Potential", "Quintessence", "Lower Potential", "Source"];
    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 10] = [
        ("Range of Potential", "Higher Potential", "Lower Potential"),
        ("Range of Significance", "Purpose", "Source"),
        ("Aspiration", "Quintessence", "Higher Potential"),
        ("Operation", "Quintessence", "Lower Potential"),
        ("Output", "Higher Potential", "Purpose"),
        ("Input", "Lower Potential", "Source"),
        ("Qualitative Match", "Quintessence", "Purpose"),
        ("Quantitative Match", "Quintessence", "Source"),
        ("Form", "Lower Potential", "Purpose"),
        ("Function", "Higher Potential", "Source"),
    ];

    // Topology (from by_topology/k5.rs)
    pub const NODES: [Node; 5] = [0, 1, 2, 3, 4];
    pub const EDGES: [Edge; 10] = [
        (0, 1), // Purpose-Higher Potential
        (1, 2), // Higher Potential-Quintessence
        (2, 3), // Quintessence-Lower Potential
        (3, 4), // Lower Potential-Source
        (4, 0), // Source-Purpose
        (0, 2), // Purpose-Quintessence
        (1, 3), // Higher Potential-Lower Potential
        (2, 4), // Quintessence-Source
        (3, 0), // Lower Potential-Purpose
        (4, 1), // Source-Higher Potential
    ];

    // Geometry (from by_geometry/k5.rs)
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
