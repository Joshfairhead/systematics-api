pub struct MonadSystem;

impl MonadSystem {
    // Metadata (homotopy types)
    pub const SYSTEM_NAME: &'static str = "Monad";
    pub const COHERENCE: &'static str = "Universality";
    pub const TERM_DESIGNATION: &'static str = "Totality";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Unity";

    // Parallel arrays - bimorphic by index (index 0 = position 1)
    pub const TERMS: [&'static str; 1] = ["Unity"];
    pub const COLOURS_HEX: [&'static str; 1] = ["#FF0000"];
    pub const COLOURS_NAME: [&'static str; 1] = ["Red"];
    pub const COORDS: [(f64, f64); 1] = [(0.0, 0.0)];

    // Connectives - reference by position INDEX
    pub const CONNECTIVES: [(&'static str, usize, usize); 0] = [];
}
