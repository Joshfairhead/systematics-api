pub struct EnneadSystem;

impl EnneadSystem {
    // Metadata (homotopy types)
    pub const SYSTEM_NAME: &'static str = "Ennead";
    pub const COHERENCE: &'static str = "Transformation";
    pub const TERM_DESIGNATION: &'static str = "Needs Research";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Needs Research";

    // Parallel arrays - bimorphic by index (index 0 = position 1, etc.)
    pub const TERMS: [&'static str; 9] = [
        "Position 1",
        "Position 2",
        "Position 3",
        "Position 4",
        "Position 5",
        "Position 6",
        "Position 7",
        "Position 8",
        "Position 9",
    ];
    pub const COLOURS_HEX: [&'static str; 9] = ["#FF0000", "#0000FF", "#FFFF00", "#099902", "#9900FF", "#FFA500", "#00FFFF", "#8B4513", "#FF00FF"];
    pub const COLOURS_NAME: [&'static str; 9] = ["Red", "Blue", "Yellow", "Green", "Purple", "Orange", "Light Blue", "Brown", "Magenta"];
    pub const COORDS: [(f64, f64); 9] = [
        (0.0, 1.0),                        // Position 1 (top)
        (0.64278760968, 0.76604444311),    // Position 2
        (0.98480775301, 0.17364817767),    // Position 3
        (0.86602540378, -0.5),             // Position 4
        (0.34202014333, -0.93969262079),   // Position 5
        (-0.34202014333, -0.93969262079),  // Position 6
        (-0.86602540378, -0.5),            // Position 7
        (-0.98480775301, 0.17364817767),   // Position 8
        (-0.64278760968, 0.76604444311),   // Position 9
    ];

    // Connectives - reference by position INDEX (complete graph: 36 edges)
    pub const CONNECTIVES: [(&'static str, usize, usize); 36] = [
        ("Needs Research 1-2", 0, 1),
        ("Needs Research 1-3", 0, 2),
        ("Needs Research 1-4", 0, 3),
        ("Needs Research 1-5", 0, 4),
        ("Needs Research 1-6", 0, 5),
        ("Needs Research 1-7", 0, 6),
        ("Needs Research 1-8", 0, 7),
        ("Needs Research 1-9", 0, 8),
        ("Needs Research 2-3", 1, 2),
        ("Needs Research 2-4", 1, 3),
        ("Needs Research 2-5", 1, 4),
        ("Needs Research 2-6", 1, 5),
        ("Needs Research 2-7", 1, 6),
        ("Needs Research 2-8", 1, 7),
        ("Needs Research 2-9", 1, 8),
        ("Needs Research 3-4", 2, 3),
        ("Needs Research 3-5", 2, 4),
        ("Needs Research 3-6", 2, 5),
        ("Needs Research 3-7", 2, 6),
        ("Needs Research 3-8", 2, 7),
        ("Needs Research 3-9", 2, 8),
        ("Needs Research 4-5", 3, 4),
        ("Needs Research 4-6", 3, 5),
        ("Needs Research 4-7", 3, 6),
        ("Needs Research 4-8", 3, 7),
        ("Needs Research 4-9", 3, 8),
        ("Needs Research 5-6", 4, 5),
        ("Needs Research 5-7", 4, 6),
        ("Needs Research 5-8", 4, 7),
        ("Needs Research 5-9", 4, 8),
        ("Needs Research 6-7", 5, 6),
        ("Needs Research 6-8", 5, 7),
        ("Needs Research 6-9", 5, 8),
        ("Needs Research 7-8", 6, 7),
        ("Needs Research 7-9", 6, 8),
        ("Needs Research 8-9", 7, 8),
    ];
}
