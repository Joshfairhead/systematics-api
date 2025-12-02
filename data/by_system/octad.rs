use crate::core::state_manager::{Coordinates, Index};

pub struct OctadSystem;

impl OctadSystem {
    // Metadata
    pub const SYSTEM_NAME: &'static str = "Octad";
    pub const COHERENCE_ATTRIBUTE: &'static str = "Self Sufficiency";
    pub const TERM_DESIGNATION: &'static str = "Elements";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Components";
    pub const SOURCE: &'static str = "Qualsystems Book";

    // Vocabulary (from by_vocabulary/k8.rs)
    pub const TERM_CHARACTERS: [&'static str; 8] = [
        "Smallest Significant Holon",
        "Critical Functions",
        "Supportive Platform",
        "Necessary Resourcing",
        "Integrative Totality",
        "Inherent Values",
        "Intrinsic Nature",
        "Organisational Modes"
    ];
    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 28] = [
        ("Component1", "Smallest Significant Holon", "Critical Functions"),
        ("Component2", "Smallest Significant Holon", "Supportive Platform"),
        ("Component3", "Smallest Significant Holon", "Necessary Resourcing"),
        ("Component4", "Smallest Significant Holon", "Integrative Totality"),
        ("Component5", "Smallest Significant Holon", "Inherent Values"),
        ("Component6", "Smallest Significant Holon", "Intrinsic Nature"),
        ("Component7", "Smallest Significant Holon", "Organisational Modes"),
        ("Component8", "Critical Functions", "Supportive Platform"),
        ("Component9", "Critical Functions", "Necessary Resourcing"),
        ("Component10", "Critical Functions", "Integrative Totality"),
        ("Component11", "Critical Functions", "Inherent Values"),
        ("Component12", "Critical Functions", "Intrinsic Nature"),
        ("Component13", "Critical Functions", "Organisational Modes"),
        ("Component14", "Supportive Platform", "Necessary Resourcing"),
        ("Component15", "Supportive Platform", "Integrative Totality"),
        ("Component16", "Supportive Platform", "Inherent Values"),
        ("Component17", "Supportive Platform", "Intrinsic Nature"),
        ("Component18", "Supportive Platform", "Organisational Modes"),
        ("Component19", "Necessary Resourcing", "Integrative Totality"),
        ("Component20", "Necessary Resourcing", "Inherent Values"),
        ("Component21", "Necessary Resourcing", "Intrinsic Nature"),
        ("Component22", "Necessary Resourcing", "Organisational Modes"),
        ("Component23", "Integrative Totality", "Inherent Values"),
        ("Component24", "Integrative Totality", "Intrinsic Nature"),
        ("Component25", "Integrative Totality", "Organisational Modes"),
        ("Component26", "Inherent Values", "Intrinsic Nature"),
        ("Component27", "Inherent Values", "Organisational Modes"),
        ("Component28", "Intrinsic Nature", "Organisational Modes"),
    ];

    // Topology (from by_topology/k8.rs)
    pub const INDEX: [Index; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    pub const INDICES: [(Index, Index); 28] = [
        (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7),
        (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7),
        (2, 3), (2, 4), (2, 5), (2, 6), (2, 7),
        (3, 4), (3, 5), (3, 6), (3, 7),
        (4, 5), (4, 6), (4, 7),
        (5, 6), (5, 7),
        (6, 7),
    ];

    // Geometry (from by_geometry/k8.rs)
    pub const POINTS: [Coordinates; 8] = [
        Coordinates { x: 1.0, y: 0.0, z: None },                        // 0: Smallest Significant Holon (right, middle)
        Coordinates { x: 0.70710678118, y: -0.70710678118, z: None },   // 1: Critical Functions (lower right)
        Coordinates { x: 0.0, y: -1.0, z: None },                       // 2: Supportive Platform (bottom)
        Coordinates { x: -0.70710678118, y: -0.70710678118, z: None },  // 3: Necessary Resourcing (lower left)
        Coordinates { x: -1.0, y: 0.0, z: None },                       // 4: Integrative Totality (left, middle)
        Coordinates { x: -0.70710678118, y: 0.70710678118, z: None },   // 5: Inherent Values (upper left)
        Coordinates { x: 0.0, y: 1.0, z: None },                        // 6: Intrinsic Nature (top)
        Coordinates { x: 0.70710678118, y: 0.70710678118, z: None },    // 7: Organisational Modes (upper right)
    ];
    pub const LINES: [(Coordinates, Coordinates); 28] = [
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: 0.70710678118, y: -0.70710678118, z: None }),
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }),
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: -0.70710678118, y: -0.70710678118, z: None }),
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: -1.0, y: 0.0, z: None }),
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: -0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: 0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: 0.70710678118, y: -0.70710678118, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }),
        (Coordinates { x: 0.70710678118, y: -0.70710678118, z: None }, Coordinates { x: -0.70710678118, y: -0.70710678118, z: None }),
        (Coordinates { x: 0.70710678118, y: -0.70710678118, z: None }, Coordinates { x: -1.0, y: 0.0, z: None }),
        (Coordinates { x: 0.70710678118, y: -0.70710678118, z: None }, Coordinates { x: -0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: 0.70710678118, y: -0.70710678118, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: 0.70710678118, y: -0.70710678118, z: None }, Coordinates { x: 0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: -0.70710678118, y: -0.70710678118, z: None }),
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: -1.0, y: 0.0, z: None }),
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: -0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: 0.0, y: -1.0, z: None }, Coordinates { x: 0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: -0.70710678118, y: -0.70710678118, z: None }, Coordinates { x: -1.0, y: 0.0, z: None }),
        (Coordinates { x: -0.70710678118, y: -0.70710678118, z: None }, Coordinates { x: -0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: -0.70710678118, y: -0.70710678118, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: -0.70710678118, y: -0.70710678118, z: None }, Coordinates { x: 0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: -1.0, y: 0.0, z: None }, Coordinates { x: -0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: -1.0, y: 0.0, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: -1.0, y: 0.0, z: None }, Coordinates { x: 0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: -0.70710678118, y: 0.70710678118, z: None }, Coordinates { x: 0.0, y: 1.0, z: None }),
        (Coordinates { x: -0.70710678118, y: 0.70710678118, z: None }, Coordinates { x: 0.70710678118, y: 0.70710678118, z: None }),
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: 0.70710678118, y: 0.70710678118, z: None }),
    ];
}
