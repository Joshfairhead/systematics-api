pub struct TriadSystem;

impl TriadSystem {
    // Metadata (homotopy types)
    pub const SYSTEM_NAME: &'static str = "Triad";
    pub const COHERENCE: &'static str = "Dynamism";
    pub const TERM_DESIGNATION: &'static str = "Impulses";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Acts";

    // Parallel arrays - bimorphic by index (index 0 = position 1, etc.)
    pub const TERMS: [&'static str; 3] = ["Will", "Function", "Being"];
    pub const COLOURS_HEX: [&'static str; 3] = ["#FF0000", "#0000FF", "#FFFF00"];
    pub const COLOURS_NAME: [&'static str; 3] = ["Red", "Blue", "Yellow"];
    pub const COORDS: [(f64, f64); 3] = [(0.0, 1.0), (0.0, -1.0), (1.0, 0.0)];

    // Connectives - reference by position INDEX
    pub const CONNECTIVES: [(&'static str, usize, usize); 3] = [
        ("Act1", 0, 1),   // Will→Function (positions 1→2)
        ("Act2", 1, 2),   // Function→Being (positions 2→3)
        ("Act3", 2, 0),   // Being→Will (positions 3→1)
    ];
}
