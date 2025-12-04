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
    pub const TERM_CHARACTERS: [&'static str; 6] = ["Resources", "Values", "Options", "Criteria", "Facts", "Priorities"];
    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 15] = [
        ("Step1", "Resources", "Values"),
        ("Step2", "Resources", "Options"),
        ("Step3", "Resources", "Criteria"),
        ("Step4", "Resources", "Facts"),
        ("Step5", "Resources", "Priorities"),
        ("Step6", "Values", "Options"),
        ("Step7", "Values", "Criteria"),
        ("Step8", "Values", "Facts"),
        ("Step9", "Values", "Priorities"),
        ("Step10", "Options", "Criteria"),
        ("Step11", "Options", "Facts"),
        ("Step12", "Options", "Priorities"),
        ("Step13", "Criteria", "Facts"),
        ("Step14", "Criteria", "Priorities"),
        ("Step15", "Facts", "Priorities"),
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
    // Points ordered to match vocabulary: Resources, Values, Options, Criteria, Facts, Priorities
    // Positioned clockwise from top: Values, Options, Criteria, Facts, Priorities, Resources
    pub const POINTS: [Coordinates; 6] = [
        Coordinates { x: -0.866, y: 0.5, z: None },         // index 0: Resources (upper left)
        Coordinates { x: 0.0, y: 1.0, z: None },            // index 1: Values (top)
        Coordinates { x: 0.866, y: 0.5, z: None },          // index 2: Options (upper right)
        Coordinates { x: 0.866, y: -0.5, z: None },         // index 3: Criteria (lower right)
        Coordinates { x: 0.0, y: -1.0, z: None },           // index 4: Facts (bottom)
        Coordinates { x: -0.866, y: -0.5, z: None },        // index 5: Priorities (lower left)
    ];
    pub const LINES: [(Coordinates, Coordinates); 15] = [
        // Edge 0-1: Resources to Values
        (Coordinates { x: -0.866, y: 0.5, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        // Edge 0-2: Resources to Options
        (Coordinates { x: -0.866, y: 0.5, z: None }, Coordinates { x: 0.866, y: 0.5, z: None }),
        // Edge 0-3: Resources to Criteria
        (Coordinates { x: -0.866, y: 0.5, z: None }, Coordinates { x: 0.866, y: -0.5, z: None }),
        // Edge 0-4: Resources to Facts
        (Coordinates { x: -0.866, y: 0.5, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }),
        // Edge 0-5: Resources to Priorities
        (Coordinates { x: -0.866, y: 0.5, z: None }, Coordinates { x: -0.866, y: -0.5, z: None }),
        // Edge 1-2: Values to Options
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: 0.866, y: 0.5, z: None }),
        // Edge 1-3: Values to Criteria
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: 0.866, y: -0.5, z: None }),
        // Edge 1-4: Values to Facts
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }),
        // Edge 1-5: Values to Priorities
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: -0.866, y: -0.5, z: None }),
        // Edge 2-3: Options to Criteria
        (Coordinates { x: 0.866, y: 0.5, z: None }, Coordinates { x: 0.866, y: -0.5, z: None }),
        // Edge 2-4: Options to Facts
        (Coordinates { x: 0.866, y: 0.5, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }),
        // Edge 2-5: Options to Priorities
        (Coordinates { x: 0.866, y: 0.5, z: None }, Coordinates { x: -0.866, y: -0.5, z: None }),
        // Edge 3-4: Criteria to Facts
        (Coordinates { x: 0.866, y: -0.5, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }),
        // Edge 3-5: Criteria to Priorities
        (Coordinates { x: 0.866, y: -0.5, z: None }, Coordinates { x: -0.866, y: -0.5, z: None }),
        // Edge 4-5: Facts to Priorities
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: -0.866, y: -0.5, z: None }),
    ];
}
