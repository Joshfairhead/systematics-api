pub struct PentadSystem;

impl PentadSystem {
    // Metadata (homotopy types)
    pub const SYSTEM_NAME: &'static str = "Pentad";
    pub const COHERENCE: &'static str = "Significance and Potential";
    pub const TERM_DESIGNATION: &'static str = "Limits";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Mutualities";

    // Parallel arrays - bimorphic by index (index 0 = position 1, etc.)
    // Position 1=Quintessence, 2=Source, 3=Higher Potential, 4=Lower Potential, 5=Purpose
    pub const TERMS: [&'static str; 5] = ["Quintessence", "Source", "Higher Potential", "Lower Potential", "Purpose"];
    pub const COLOURS_HEX: [&'static str; 5] = ["#FF0000", "#0000FF", "#FFFF00", "#099902", "#9900FF"];
    pub const COLOURS_NAME: [&'static str; 5] = ["Red", "Blue", "Yellow", "Green", "Purple"];
    pub const COORDS: [(f64, f64); 5] = [(-0.75, 0.0), (1.0, -0.75), (0.0, 0.5), (0.0, -0.5), (1.0, 0.75)];

    // Connectives - reference by position INDEX (complete graph: 10 edges)
    pub const CONNECTIVES: [(&'static str, usize, usize); 10] = [
        ("Quantitative Match", 0, 1),         // Quintessence→Source (1→2)
        ("Aspiration", 0, 2),                 // Quintessence→Higher Potential (1→3)
        ("Operation", 0, 3),                  // Quintessence→Lower Potential (1→4)
        ("Qualitative Match", 0, 4),          // Quintessence→Purpose (1→5)
        ("Function", 1, 2),                   // Source→Higher Potential (2→3)
        ("Input", 1, 3),                      // Source→Lower Potential (2→4)
        ("Range of Significance", 1, 4),      // Source→Purpose (2→5)
        ("Range of Potential", 2, 3),         // Higher Potential→Lower Potential (3→4)
        ("Output", 2, 4),                     // Higher Potential→Purpose (3→5)
        ("Form", 3, 4),                       // Lower Potential→Purpose (4→5)
    ];
}
