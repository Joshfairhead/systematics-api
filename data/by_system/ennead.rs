use crate::core::topology::{Node, Edge};
use crate::core::geometry::Coordinates;

pub struct EnneadSystem;

impl EnneadSystem {
    // Metadata
    pub const SYSTEM_NAME: &'static str = "Ennead";
    pub const COHERENCE_ATTRIBUTE: &'static str = "Transformation";
    pub const TERM_DESIGNATION: &'static str = "Needs Research";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Needs Research";
    pub const SOURCE: &'static str = "Elementary Systematics";

    // Vocabulary (from by_vocabulary/k9.rs)
    pub const TERM_CHARACTERS: [&'static str; 9] = [
        "Position1",
        "Position2",
        "Position3",
        "Position4",
        "Position5",
        "Position6",
        "Position7",
        "Position8",
        "Position9"
    ];
    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 36] = [
        ("Needs Research1", "Position1", "Position2"),
        ("Needs Research2", "Position1", "Position3"),
        ("Needs Research3", "Position1", "Position4"),
        ("Needs Research4", "Position1", "Position5"),
        ("Needs Research5", "Position1", "Position6"),
        ("Needs Research6", "Position1", "Position7"),
        ("Needs Research7", "Position1", "Position8"),
        ("Needs Research8", "Position1", "Position9"),
        ("Needs Research9", "Position2", "Position3"),
        ("Needs Research10", "Position2", "Position4"),
        ("Needs Research11", "Position2", "Position5"),
        ("Needs Research12", "Position2", "Position6"),
        ("Needs Research13", "Position2", "Position7"),
        ("Needs Research14", "Position2", "Position8"),
        ("Needs Research15", "Position2", "Position9"),
        ("Needs Research16", "Position3", "Position4"),
        ("Needs Research17", "Position3", "Position5"),
        ("Needs Research18", "Position3", "Position6"),
        ("Needs Research19", "Position3", "Position7"),
        ("Needs Research20", "Position3", "Position8"),
        ("Needs Research21", "Position3", "Position9"),
        ("Needs Research22", "Position4", "Position5"),
        ("Needs Research23", "Position4", "Position6"),
        ("Needs Research24", "Position4", "Position7"),
        ("Needs Research25", "Position4", "Position8"),
        ("Needs Research26", "Position4", "Position9"),
        ("Needs Research27", "Position5", "Position6"),
        ("Needs Research28", "Position5", "Position7"),
        ("Needs Research29", "Position5", "Position8"),
        ("Needs Research30", "Position5", "Position9"),
        ("Needs Research31", "Position6", "Position7"),
        ("Needs Research32", "Position6", "Position8"),
        ("Needs Research33", "Position6", "Position9"),
        ("Needs Research34", "Position7", "Position8"),
        ("Needs Research35", "Position7", "Position9"),
        ("Needs Research36", "Position8", "Position9"),
    ];

    // Topology (from by_topology/k9.rs)
    pub const NODES: [Node; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    pub const EDGES: [Edge; 36] = [
        (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8),
        (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7), (1, 8),
        (2, 3), (2, 4), (2, 5), (2, 6), (2, 7), (2, 8),
        (3, 4), (3, 5), (3, 6), (3, 7), (3, 8),
        (4, 5), (4, 6), (4, 7), (4, 8),
        (5, 6), (5, 7), (5, 8),
        (6, 7), (6, 8),
        (7, 8),
    ];

    // Geometry (from by_geometry/k9.rs)
    pub const POINTS: [Coordinates; 9] = [
        Coordinates { x: 0.64278760968, y: 0.76604444311, z: None },    // 0: top-right
        Coordinates { x: 0.98480775301, y: 0.17364817767, z: None },    // 1: right
        Coordinates { x: 0.86602540378, y: -0.5, z: None },             // 2: bottom-right
        Coordinates { x: 0.34202014333, y: -0.93969262079, z: None },   // 3: bottom
        Coordinates { x: -0.34202014333, y: -0.93969262079, z: None },  // 4: bottom-left
        Coordinates { x: -0.86602540378, y: -0.5, z: None },            // 5: left
        Coordinates { x: -0.98480775301, y: 0.17364817767, z: None },   // 6: top-left
        Coordinates { x: -0.64278760968, y: 0.76604444311, z: None },   // 7: top-left-2
        Coordinates { x: 0.0, y: 1.0, z: None },                        // 8: top
    ];
    pub const LINES: [(Coordinates, Coordinates); 36] = [
        (Coordinates { x: 0.64278760968, y: 0.76604444311, z: None }, Coordinates { x: 0.98480775301, y: 0.17364817767, z: None }),
        (Coordinates { x: 0.64278760968, y: 0.76604444311, z: None }, Coordinates { x: 0.86602540378, y: -0.5, z: None }),
        (Coordinates { x: 0.64278760968, y: 0.76604444311, z: None }, Coordinates { x: 0.34202014333, y: -0.93969262079, z: None }),
        (Coordinates { x: 0.64278760968, y: 0.76604444311, z: None }, Coordinates { x: -0.34202014333, y: -0.93969262079, z: None }),
        (Coordinates { x: 0.64278760968, y: 0.76604444311, z: None }, Coordinates { x: -0.86602540378, y: -0.5, z: None }),
        (Coordinates { x: 0.64278760968, y: 0.76604444311, z: None }, Coordinates { x: -0.98480775301, y: 0.17364817767, z: None }),
        (Coordinates { x: 0.64278760968, y: 0.76604444311, z: None }, Coordinates { x: -0.64278760968, y: 0.76604444311, z: None }),
        (Coordinates { x: 0.64278760968, y: 0.76604444311, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: 0.98480775301, y: 0.17364817767, z: None }, Coordinates { x: 0.86602540378, y: -0.5, z: None }),
        (Coordinates { x: 0.98480775301, y: 0.17364817767, z: None }, Coordinates { x: 0.34202014333, y: -0.93969262079, z: None }),
        (Coordinates { x: 0.98480775301, y: 0.17364817767, z: None }, Coordinates { x: -0.34202014333, y: -0.93969262079, z: None }),
        (Coordinates { x: 0.98480775301, y: 0.17364817767, z: None }, Coordinates { x: -0.86602540378, y: -0.5, z: None }),
        (Coordinates { x: 0.98480775301, y: 0.17364817767, z: None }, Coordinates { x: -0.98480775301, y: 0.17364817767, z: None }),
        (Coordinates { x: 0.98480775301, y: 0.17364817767, z: None }, Coordinates { x: -0.64278760968, y: 0.76604444311, z: None }),
        (Coordinates { x: 0.98480775301, y: 0.17364817767, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: 0.86602540378, y: -0.5, z: None }, Coordinates { x: 0.34202014333, y: -0.93969262079, z: None }),
        (Coordinates { x: 0.86602540378, y: -0.5, z: None }, Coordinates { x: -0.34202014333, y: -0.93969262079, z: None }),
        (Coordinates { x: 0.86602540378, y: -0.5, z: None }, Coordinates { x: -0.86602540378, y: -0.5, z: None }),
        (Coordinates { x: 0.86602540378, y: -0.5, z: None }, Coordinates { x: -0.98480775301, y: 0.17364817767, z: None }),
        (Coordinates { x: 0.86602540378, y: -0.5, z: None }, Coordinates { x: -0.64278760968, y: 0.76604444311, z: None }),
        (Coordinates { x: 0.86602540378, y: -0.5, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: 0.34202014333, y: -0.93969262079, z: None }, Coordinates { x: -0.34202014333, y: -0.93969262079, z: None }),
        (Coordinates { x: 0.34202014333, y: -0.93969262079, z: None }, Coordinates { x: -0.86602540378, y: -0.5, z: None }),
        (Coordinates { x: 0.34202014333, y: -0.93969262079, z: None }, Coordinates { x: -0.98480775301, y: 0.17364817767, z: None }),
        (Coordinates { x: 0.34202014333, y: -0.93969262079, z: None }, Coordinates { x: -0.64278760968, y: 0.76604444311, z: None }),
        (Coordinates { x: 0.34202014333, y: -0.93969262079, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: -0.34202014333, y: -0.93969262079, z: None }, Coordinates { x: -0.86602540378, y: -0.5, z: None }),
        (Coordinates { x: -0.34202014333, y: -0.93969262079, z: None }, Coordinates { x: -0.98480775301, y: 0.17364817767, z: None }),
        (Coordinates { x: -0.34202014333, y: -0.93969262079, z: None }, Coordinates { x: -0.64278760968, y: 0.76604444311, z: None }),
        (Coordinates { x: -0.34202014333, y: -0.93969262079, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: -0.86602540378, y: -0.5, z: None }, Coordinates { x: -0.98480775301, y: 0.17364817767, z: None }),
        (Coordinates { x: -0.86602540378, y: -0.5, z: None }, Coordinates { x: -0.64278760968, y: 0.76604444311, z: None }),
        (Coordinates { x: -0.86602540378, y: -0.5, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: -0.98480775301, y: 0.17364817767, z: None }, Coordinates { x: -0.64278760968, y: 0.76604444311, z: None }),
        (Coordinates { x: -0.98480775301, y: 0.17364817767, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: -0.64278760968, y: 0.76604444311, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
    ];
}
