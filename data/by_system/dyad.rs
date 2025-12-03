use crate::core::topology::{Node, Edge};
use crate::core::geometry::Coordinates;

pub struct DyadSystem;

impl DyadSystem {
    // Metadata
    pub const SYSTEM_NAME: &'static str = "Dyad";
    pub const COHERENCE_ATTRIBUTE: &'static str = "Complimentarity";
    pub const TERM_DESIGNATION: &'static str = "Poles";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Force";
    pub const SOURCE: &'static str = "Elementary Systematics";

    // Vocabulary
    pub const TERM_CHARACTERS: [&'static str; 2] = ["Essence", "Existence"];
    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 1] = [
        ("Force1", "Essence", "Existence"),
    ];

    // Topology
    pub const NODES: [Node; 2] = [0, 1];
    pub const EDGES: [Edge; 1] = [
        (0, 1), // Single edge
    ];

    // Geometry
    pub const POINTS: [Coordinates; 2] = [
        Coordinates { x: -1.0, y: 0.0, z: None }, // Left
        Coordinates { x: 1.0, y: 0.0, z: None },  // Right
    ];
    pub const LINES: [(Coordinates, Coordinates); 1] = [
        (Coordinates { x: -1.0, y: 0.0, z: None }, Coordinates { x: 1.0, y: 0.0, z: None }), // Single edge
    ];
}
