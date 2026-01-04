pub struct TetradSystem;

impl TetradSystem {
    // Metadata (homotopy types)
    pub const SYSTEM_NAME: &'static str = "Tetrad";
    pub const COHERENCE: &'static str = "Activity Field";
    pub const TERM_DESIGNATION: &'static str = "Sources";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Interplays";

    // Parallel arrays - bimorphic by index (index 0 = position 1, etc.)
    // Position 1=Ideal, 2=Ground, 3=Directive, 4=Instrumental
    pub const TERMS: [&'static str; 4] = ["Ideal", "Ground", "Directive", "Instrumental"];
    pub const COLOURS_HEX: [&'static str; 4] = ["#FF0000", "#0000FF", "#FFFF00", "#099902"];
    pub const COLOURS_NAME: [&'static str; 4] = ["Red", "Blue", "Yellow", "Green"];
    pub const COORDS: [(f64, f64); 4] = [(0.0, 1.0), (0.0, -1.0), (1.0, 0.0), (-1.0, 0.0)];

    // Connectives - reference by position INDEX (complete graph: 6 edges)
    pub const CONNECTIVES: [(&'static str, usize, usize); 6] = [
        ("Receptive Regard", 0, 2),           // Ideal→Directive (1→3)
        ("Effectual Compatibility", 0, 3),    // Ideal→Instrumental (1→4)
        ("Motivational Imperative", 0, 1),    // Ideal→Ground (1→2)
        ("Demonstrable Activity", 1, 2),      // Ground→Directive (2→3)
        ("Material Mastery", 1, 3),           // Ground→Instrumental (2→4)
        ("Technical Power", 2, 3),            // Directive→Instrumental (3→4)
    ];
}
