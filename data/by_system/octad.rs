pub struct OctadSystem;

impl OctadSystem {
    // Metadata (homotopy types)
    pub const SYSTEM_NAME: &'static str = "Octad";
    pub const COHERENCE: &'static str = "Self-Sufficiency";
    pub const TERM_DESIGNATION: &'static str = "Elements";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Components";

    // Parallel arrays - bimorphic by index (index 0 = position 1, etc.)
    // Position 1=Inherent Values, 2=Critical Functions, 3=Organisational Modes, 4=Necessary Resourcing,
    // 5=Intrinsic Nature, 6=Smallest Significant Holon, 7=Integrative Totality, 8=Supportive Platform
    pub const TERMS: [&'static str; 8] = [
        "Inherent Values",
        "Critical Functions",
        "Organisational Modes",
        "Necessary Resourcing",
        "Intrinsic Nature",
        "Smallest Significant Holon",
        "Integrative Totality",
        "Supportive Platform",
    ];
    pub const COLOURS_HEX: [&'static str; 8] = ["#FF0000", "#0000FF", "#FFFF00", "#099902", "#9900FF", "#FFA500", "#00FFFF", "#8B4513"];
    pub const COLOURS_NAME: [&'static str; 8] = ["Red", "Blue", "Yellow", "Green", "Purple", "Orange", "Light Blue", "Brown"];
    pub const COORDS: [(f64, f64); 8] = [
        (-0.71, 0.71),   // Inherent Values (upper left)
        (0.71, -0.71),   // Critical Functions (lower right)
        (0.71, 0.71),    // Organisational Modes (upper right)
        (-0.71, -0.71),  // Necessary Resourcing (lower left)
        (0.0, 1.0),      // Intrinsic Nature (top)
        (1.0, 0.0),      // Smallest Significant Holon (right)
        (-1.0, 0.0),     // Integrative Totality (left)
        (0.0, -1.0),     // Supportive Platform (bottom)
    ];

    // Connectives - reference by position INDEX (complete graph: 28 edges)
    pub const CONNECTIVES: [(&'static str, usize, usize); 28] = [
        ("Component 1-2", 0, 1),
        ("Component 1-3", 0, 2),
        ("Component 1-4", 0, 3),
        ("Component 1-5", 0, 4),
        ("Component 1-6", 0, 5),
        ("Component 1-7", 0, 6),
        ("Component 1-8", 0, 7),
        ("Component 2-3", 1, 2),
        ("Component 2-4", 1, 3),
        ("Component 2-5", 1, 4),
        ("Component 2-6", 1, 5),
        ("Component 2-7", 1, 6),
        ("Component 2-8", 1, 7),
        ("Component 3-4", 2, 3),
        ("Component 3-5", 2, 4),
        ("Component 3-6", 2, 5),
        ("Component 3-7", 2, 6),
        ("Component 3-8", 2, 7),
        ("Component 4-5", 3, 4),
        ("Component 4-6", 3, 5),
        ("Component 4-7", 3, 6),
        ("Component 4-8", 3, 7),
        ("Component 5-6", 4, 5),
        ("Component 5-7", 4, 6),
        ("Component 5-8", 4, 7),
        ("Component 6-7", 5, 6),
        ("Component 6-8", 5, 7),
        ("Component 7-8", 6, 7),
    ];
}
