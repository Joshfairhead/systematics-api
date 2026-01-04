pub struct HexadSystem;

impl HexadSystem {
    // Metadata (homotopy types)
    pub const SYSTEM_NAME: &'static str = "Hexad";
    pub const COHERENCE: &'static str = "Coalescence";
    pub const TERM_DESIGNATION: &'static str = "Laws";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Steps";

    // Parallel arrays - bimorphic by index (index 0 = position 1, etc.)
    // Position 1=Priorities, 2=Criteria, 3=Values, 4=Resources, 5=Options, 6=Facts
    pub const TERMS: [&'static str; 6] = ["Priorities", "Criteria", "Values", "Resources", "Options", "Facts"];
    pub const COLOURS_HEX: [&'static str; 6] = ["#FF0000", "#0000FF", "#FFFF00", "#099902", "#9900FF", "#FFA500"];
    pub const COLOURS_NAME: [&'static str; 6] = ["Red", "Blue", "Yellow", "Green", "Purple", "Orange"];
    pub const COORDS: [(f64, f64); 6] = [(-0.87, -0.5), (0.87, -0.5), (0.0, 1.0), (-0.87, 0.5), (0.87, 0.5), (0.0, -1.0)];

    // Connectives - reference by position INDEX (complete graph: 15 edges)
    pub const CONNECTIVES: [(&'static str, usize, usize); 15] = [
        ("Step14", 0, 1),   // Priorities→Criteria (1→2)
        ("Step9", 0, 2),    // Priorities→Values (1→3)
        ("Step5", 0, 3),    // Priorities→Resources (1→4)
        ("Step12", 0, 4),   // Priorities→Options (1→5)
        ("Step15", 0, 5),   // Priorities→Facts (1→6)
        ("Step7", 1, 2),    // Criteria→Values (2→3)
        ("Step3", 1, 3),    // Criteria→Resources (2→4)
        ("Step10", 1, 4),   // Criteria→Options (2→5)
        ("Step13", 1, 5),   // Criteria→Facts (2→6)
        ("Step1", 2, 3),    // Values→Resources (3→4)
        ("Step6", 2, 4),    // Values→Options (3→5)
        ("Step8", 2, 5),    // Values→Facts (3→6)
        ("Step2", 3, 4),    // Resources→Options (4→5)
        ("Step4", 3, 5),    // Resources→Facts (4→6)
        ("Step11", 4, 5),   // Options→Facts (5→6)
    ];
}
