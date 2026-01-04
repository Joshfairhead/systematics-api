pub struct DyadSystem;

impl DyadSystem {
    // Metadata (homotopy types)
    pub const SYSTEM_NAME: &'static str = "Dyad";
    pub const COHERENCE: &'static str = "Complementarity";
    pub const TERM_DESIGNATION: &'static str = "Poles";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Force";

    // Parallel arrays - bimorphic by index (index 0 = position 1, index 1 = position 2)
    pub const TERMS: [&'static str; 2] = ["Essence", "Existence"];
    pub const COLOURS_HEX: [&'static str; 2] = ["#FF0000", "#0000FF"];
    pub const COLOURS_NAME: [&'static str; 2] = ["Red", "Blue"];
    pub const COORDS: [(f64, f64); 2] = [(-1.0, 0.0), (1.0, 0.0)];

    // Connectives - reference by position INDEX
    pub const CONNECTIVES: [(&'static str, usize, usize); 1] = [
        ("Force", 0, 1),  // Essence→Existence (positions 1→2)
    ];
}
