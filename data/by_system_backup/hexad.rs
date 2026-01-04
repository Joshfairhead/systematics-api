use crate::core::topology::{Node, Edge};
use crate::core::geometry::Coordinates;

pub struct HexadSystem;

impl HexadSystem {
    // Metadata
    pub const SYSTEM_NAME: &'static str = "Hexad";
    pub const COHERENCE_ATTRIBUTE: &'static str = "Coalescence";
    pub const TERM_DESIGNATION: &'static str = "Laws";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Steps";
    pub const SOURCE: &'static str = "Elementary Systematics";

    // Vocabulary (from by_vocabulary/k6.rs)
    // Semantic ordering: Priorities (1/Red), Criteria (2/Blue), Values (3/Yellow), Resources (4/Green), Options (5/Purple), Facts (6/Orange) when displayed with one-based indexing
    pub const TERM_CHARACTERS: [&'static str; 6] = ["Priorities", "Criteria", "Values", "Resources", "Options", "Facts"];

    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 15] = [
        ("Step15", "Priorities", "Facts"),
        ("Step14", "Priorities", "Criteria"),
        ("Step12", "Priorities", "Options"),
        ("Step9", "Priorities", "Values"),
        ("Step5", "Priorities", "Resources"),
        ("Step13", "Facts", "Criteria"),
        ("Step11", "Facts", "Options"),
        ("Step8", "Facts", "Values"),
        ("Step4", "Facts", "Resources"),
        ("Step10", "Criteria", "Options"),
        ("Step7", "Criteria", "Values"),
        ("Step3", "Criteria", "Resources"),
        ("Step6", "Options", "Values"),
        ("Step2", "Options", "Resources"),
        ("Step1", "Values", "Resources"),
    ];

    // Topology (from by_topology/k6.rs)
    pub const NODES: [Node; 6] = [0, 1, 2, 3, 4, 5];
    pub const EDGES: [Edge; 15] = [
        (0, 1), (0, 2), (0, 3), (0, 4), (0, 5),
        (1, 2), (1, 3), (1, 4), (1, 5),
        (2, 3), (2, 4), (2, 5),
        (3, 4), (3, 5),
        (4, 5),
    ];

    // Geometry (from by_geometry/k6.rs)
    // Coordinates reordered to match semantic vocabulary ordering (reversed from original)
    pub const POINTS: [Coordinates; 6] = [
        Coordinates { x: -0.866, y: -0.5, z: None },        // index 0: Priorities (lower left)
        Coordinates { x: 0.866, y: -0.5, z: None },         // index 1: Criteria (lower right)
        Coordinates { x: 0.0, y: 1.0, z: None },            // index 2: Values (top)
        Coordinates { x: -0.866, y: 0.5, z: None },         // index 3: Resources (upper left)
        Coordinates { x: 0.866, y: 0.5, z: None },          // index 4: Options (upper right)
        Coordinates { x: 0.0, y: -1.0, z: None },           // index 5: Facts (bottom)
    ];
    pub const LINES: [(Coordinates, Coordinates); 15] = [
        // Edge 0-1: Priorities to Facts
        (Coordinates { x: -0.866, y: -0.5, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }),
        // Edge 0-2: Priorities to Criteria
        (Coordinates { x: -0.866, y: -0.5, z: None }, Coordinates { x: 0.866, y: -0.5, z: None }),
        // Edge 0-3: Priorities to Options
        (Coordinates { x: -0.866, y: -0.5, z: None }, Coordinates { x: 0.866, y: 0.5, z: None }),
        // Edge 0-4: Priorities to Values
        (Coordinates { x: -0.866, y: -0.5, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        // Edge 0-5: Priorities to Resources
        (Coordinates { x: -0.866, y: -0.5, z: None }, Coordinates { x: -0.866, y: 0.5, z: None }),
        // Edge 1-2: Facts to Criteria
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: 0.866, y: -0.5, z: None }),
        // Edge 1-3: Facts to Options
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: 0.866, y: 0.5, z: None }),
        // Edge 1-4: Facts to Values
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        // Edge 1-5: Facts to Resources
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: -0.866, y: 0.5, z: None }),
        // Edge 2-3: Criteria to Options
        (Coordinates { x: 0.866, y: -0.5, z: None }, Coordinates { x: 0.866, y: 0.5, z: None }),
        // Edge 2-4: Criteria to Values
        (Coordinates { x: 0.866, y: -0.5, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        // Edge 2-5: Criteria to Resources
        (Coordinates { x: 0.866, y: -0.5, z: None }, Coordinates { x: -0.866, y: 0.5, z: None }),
        // Edge 3-4: Options to Values
        (Coordinates { x: 0.866, y: 0.5, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        // Edge 3-5: Options to Resources
        (Coordinates { x: 0.866, y: 0.5, z: None }, Coordinates { x: -0.866, y: 0.5, z: None }),
        // Edge 4-5: Values to Resources
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: -0.866, y: 0.5, z: None }),
    ];
}
