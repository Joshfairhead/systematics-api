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
    // Semantic ordering: Quintessence (1), Source (2), Higher Potential (3), Lower Potential (4), Purpose (5) when displayed with one-based indexing
    pub const TERM_CHARACTERS: [&'static str; 5] = ["Quintessence", "Source", "Higher Potential", "Lower Potential", "Purpose"];

    // Index mapping: sequential (position → same index)
    pub const INDEX_MAPPING: [u8; 5] = [1, 2, 3, 4, 5];

    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 10] = [
        ("Input", "Source", "Lower Potential"),
        ("Quantitative Match", "Source", "Quintessence"),
        ("Function", "Source", "Higher Potential"),
        ("Range of Significance", "Source", "Purpose"),
        ("Operation", "Lower Potential", "Quintessence"),
        ("Range of Potential", "Lower Potential", "Higher Potential"),
        ("Form", "Lower Potential", "Purpose"),
        ("Aspiration", "Quintessence", "Higher Potential"),
        ("Qualitative Match", "Quintessence", "Purpose"),
        ("Output", "Higher Potential", "Purpose"),
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
    // Coordinates reordered to match semantic vocabulary ordering (reversed from original)
    pub const POINTS: [Coordinates; 5] = [
        Coordinates { x: -0.75, y: 0.0, z: None },   // 0: Quintessence (left-center, middle)
        Coordinates { x: 1.0, y: -0.75, z: None },   // 1: Source (right, bottom)
        Coordinates { x: 0.0, y: 0.5, z: None },     // 2: Higher Potential (center, upper)
        Coordinates { x: 0.0, y: -0.5, z: None },    // 3: Lower Potential (center, lower)
        Coordinates { x: 1.0, y: 0.75, z: None },    // 4: Purpose (right, top)
    ];
    pub const LINES: [(Coordinates, Coordinates); 10] = [
        (Coordinates { x: 1.0, y: -0.75, z: None }, Coordinates { x: 0.0, y: -0.5, z: None }), // Source-Lower Potential (edge 0-1)
        (Coordinates { x: 1.0, y: -0.75, z: None }, Coordinates { x: -0.75, y: 0.0, z: None }), // Source-Quintessence (edge 0-2)
        (Coordinates { x: 1.0, y: -0.75, z: None }, Coordinates { x: 0.0, y: 0.5, z: None }), // Source-Higher Potential (edge 0-3)
        (Coordinates { x: 1.0, y: -0.75, z: None }, Coordinates { x: 1.0, y: 0.75, z: None }), // Source-Purpose (edge 0-4)
        (Coordinates { x: 0.0, y: -0.5, z: None }, Coordinates { x: -0.75, y: 0.0, z: None }), // Lower Potential-Quintessence (edge 1-2)
        (Coordinates { x: 0.0, y: -0.5, z: None }, Coordinates { x: 0.0, y: 0.5, z: None }), // Lower Potential-Higher Potential (edge 1-3)
        (Coordinates { x: 0.0, y: -0.5, z: None }, Coordinates { x: 1.0, y: 0.75, z: None }), // Lower Potential-Purpose (edge 1-4)
        (Coordinates { x: -0.75, y: 0.0, z: None }, Coordinates { x: 0.0, y: 0.5, z: None }), // Quintessence-Higher Potential (edge 2-3)
        (Coordinates { x: -0.75, y: 0.0, z: None }, Coordinates { x: 1.0, y: 0.75, z: None }), // Quintessence-Purpose (edge 2-4)
        (Coordinates { x: 0.0, y: 0.5, z: None }, Coordinates { x: 1.0, y: 0.75, z: None }), // Higher Potential-Purpose (edge 3-4)
    ];
}
