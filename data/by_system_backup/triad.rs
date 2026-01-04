use crate::core::topology::{Node, Edge};
use crate::core::geometry::Coordinates;

pub struct TriadSystem;

impl TriadSystem {
    // Metadata
    pub const SYSTEM_NAME: &'static str = "Triad";
    pub const COHERENCE_ATTRIBUTE: &'static str = "Dynamism";
    pub const TERM_DESIGNATION: &'static str = "Impulses";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Acts";
    pub const SOURCE: &'static str = "Elementary Systematics";

    // Vocabulary
    // Semantic ordering: Will (1), Function (2), Being (3) when displayed with one-based indexing
    pub const TERM_CHARACTERS: [&'static str; 3] = ["Will", "Function", "Being"];

    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 3] = [
        ("Act1", "Function", "Being"),
        ("Act2", "Being", "Will"),
        ("Act3", "Will", "Function"),
    ];

    // Topology
    pub const NODES: [Node; 3] = [0, 1, 2];
    pub const EDGES: [Edge; 3] = [
        (0, 1),  // Will-Function (vertical left)
        (1, 2),  // Function-Being (bottom right)
        (2, 0),  // Being-Will (top right)
    ];

    // Geometry
    // Coordinates reordered to match semantic vocabulary ordering
    pub const POINTS: [Coordinates; 3] = [
        Coordinates { x: 0.0, y: 1.0, z: None },   // 0: Will (top left)
        Coordinates { x: 0.0, y: -1.0, z: None },  // 1: Function (bottom left)
        Coordinates { x: 1.0, y: 0.0, z: None },   // 2: Being (right, midpoint vertically)
    ];
    pub const LINES: [(Coordinates, Coordinates); 3] = [
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: 1.0, y: 0.0, z: None }),  // Function-Being (edge 0-1)
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),   // Being-Will (edge 1-2)
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }),  // Will-Function (edge 2-0)
    ];
}
