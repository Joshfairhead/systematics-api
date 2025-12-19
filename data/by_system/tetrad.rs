use crate::core::topology::{Node, Edge};
use crate::core::geometry::Coordinates;

pub struct TetradSystem;

impl TetradSystem {
    // Metadata
    pub const SYSTEM_NAME: &'static str = "Tetrad";
    pub const COHERENCE_ATTRIBUTE: &'static str = "Activity Field";
    pub const TERM_DESIGNATION: &'static str = "Sources";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Interplays";
    pub const SOURCE: &'static str = "Elementary Systematics";

    // Vocabulary
    // Semantic ordering: Ideal (1), Ground (2), Directive (3), Instrumental (4) when displayed with one-based indexing
    pub const TERM_CHARACTERS: [&'static str; 4] = ["Ideal", "Ground", "Directive", "Instrumental"];

    // Index mapping: sequential (position → same index)
    pub const INDEX_MAPPING: [u8; 4] = [1, 2, 3, 4];

    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 6] = [
        ("Technical Power", "Ground", "Instrumental"),
        ("Material Mastery", "Ground", "Directive"),
        ("Motivational Imperative", "Ground", "Ideal"),
        ("Demonstrable Activity", "Instrumental", "Directive"),
        ("Effectual Compatibility", "Instrumental", "Ideal"),
        ("Receptive Regard", "Directive", "Ideal"),
    ];

    // Topology
    pub const NODES: [Node; 4] = [0, 1, 2, 3];
    pub const EDGES: [Edge; 6] = [
        (0, 1),  // ideal-directive
        (0, 2),  // ideal-instrumental
        (0, 3),  // ideal-ground
        (1, 2),  // directive-instrumental
        (1, 3),  // directive-ground
        (2, 3),  // instrumental-ground
    ];

    // Geometry
    // Coordinates reordered to match semantic vocabulary ordering (reversed from original)
    pub const POINTS: [Coordinates; 4] = [
        Coordinates { x: 0.0, y: 1.0, z: None },   // 0: ideal (top)
        Coordinates { x: 0.0, y: -1.0, z: None },  // 1: ground (bottom)
        Coordinates { x: 1.0, y: 0.0, z: None },   // 2: directive (right)
        Coordinates { x: -1.0, y: 0.0, z: None },  // 3: instrumental (left)
    ];
    pub const LINES: [(Coordinates, Coordinates); 6] = [
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: -1.0, y: 0.0, z: None }),  // ground-instrumental (edge 0-1)
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: 1.0, y: 0.0, z: None }),  // ground-directive (edge 0-2)
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),  // ground-ideal (edge 0-3)
        (Coordinates { x: -1.0, y: 0.0, z: None }, Coordinates { x: 1.0, y: 0.0, z: None }),  // instrumental-directive (edge 1-2)
        (Coordinates { x: -1.0, y: 0.0, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),  // instrumental-ideal (edge 1-3)
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),  // directive-ideal (edge 2-3)
    ];
}
