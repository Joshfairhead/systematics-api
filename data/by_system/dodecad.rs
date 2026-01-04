pub struct DodecadSystem;

impl DodecadSystem {
    // Metadata (homotopy types)
    pub const SYSTEM_NAME: &'static str = "Dodecad";
    pub const COHERENCE: &'static str = "Perfection";
    pub const TERM_DESIGNATION: &'static str = "Needs Research";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Needs Research";

    // Parallel arrays - bimorphic by index (index 0 = position 1, etc.)
    // Position 1=Wholeness, 2=Polarity, 3=Relatedness, 4=Subsistence,
    // 5=Potentiality, 6=Repetition, 7=Structure, 8=Individuality,
    // 9=Pattern, 10=Creativity, 11=Domination, 12=Autocracy
    pub const TERMS: [&'static str; 12] = [
        "Wholeness",
        "Polarity",
        "Relatedness",
        "Subsistence",
        "Potentiality",
        "Repetition",
        "Structure",
        "Individuality",
        "Pattern",
        "Creativity",
        "Domination",
        "Autocracy",
    ];
    pub const COLOURS_HEX: [&'static str; 12] = ["#FF0000", "#0000FF", "#FFFF00", "#099902", "#9900FF", "#FFA500", "#00FFFF", "#8B4513", "#FF00FF", "#FFFFFF", "#C0C0C0", "#FFD700"];
    pub const COLOURS_NAME: [&'static str; 12] = ["Red", "Blue", "Yellow", "Green", "Purple", "Orange", "Light Blue", "Brown", "Magenta", "White", "Silver", "Gold"];
    pub const COORDS: [(f64, f64); 12] = [
        (0.86602540378, -0.5),             // Position 1: Wholeness
        (-0.5, -0.86602540378),            // Position 2: Polarity
        (0.5, -0.86602540378),             // Position 3: Relatedness
        (0.0, -1.0),                       // Position 4: Subsistence
        (-0.86602540378, -0.5),            // Position 5: Potentiality
        (-1.0, 0.0),                       // Position 6: Repetition
        (1.0, 0.0),                        // Position 7: Structure
        (0.86602540378, 0.5),              // Position 8: Individuality
        (-0.86602540378, 0.5),             // Position 9: Pattern
        (0.5, 0.86602540378),              // Position 10: Creativity
        (-0.5, 0.86602540378),             // Position 11: Domination
        (0.0, 1.0),                        // Position 12: Autocracy
    ];

    // Connectives - reference by position INDEX (complete graph: 66 edges)
    pub const CONNECTIVES: [(&'static str, usize, usize); 66] = [
        ("Needs Research 1-2", 0, 1),
        ("Needs Research 1-3", 0, 2),
        ("Needs Research 1-4", 0, 3),
        ("Needs Research 1-5", 0, 4),
        ("Needs Research 1-6", 0, 5),
        ("Needs Research 1-7", 0, 6),
        ("Needs Research 1-8", 0, 7),
        ("Needs Research 1-9", 0, 8),
        ("Needs Research 1-10", 0, 9),
        ("Needs Research 1-11", 0, 10),
        ("Needs Research 1-12", 0, 11),
        ("Needs Research 2-3", 1, 2),
        ("Needs Research 2-4", 1, 3),
        ("Needs Research 2-5", 1, 4),
        ("Needs Research 2-6", 1, 5),
        ("Needs Research 2-7", 1, 6),
        ("Needs Research 2-8", 1, 7),
        ("Needs Research 2-9", 1, 8),
        ("Needs Research 2-10", 1, 9),
        ("Needs Research 2-11", 1, 10),
        ("Needs Research 2-12", 1, 11),
        ("Needs Research 3-4", 2, 3),
        ("Needs Research 3-5", 2, 4),
        ("Needs Research 3-6", 2, 5),
        ("Needs Research 3-7", 2, 6),
        ("Needs Research 3-8", 2, 7),
        ("Needs Research 3-9", 2, 8),
        ("Needs Research 3-10", 2, 9),
        ("Needs Research 3-11", 2, 10),
        ("Needs Research 3-12", 2, 11),
        ("Needs Research 4-5", 3, 4),
        ("Needs Research 4-6", 3, 5),
        ("Needs Research 4-7", 3, 6),
        ("Needs Research 4-8", 3, 7),
        ("Needs Research 4-9", 3, 8),
        ("Needs Research 4-10", 3, 9),
        ("Needs Research 4-11", 3, 10),
        ("Needs Research 4-12", 3, 11),
        ("Needs Research 5-6", 4, 5),
        ("Needs Research 5-7", 4, 6),
        ("Needs Research 5-8", 4, 7),
        ("Needs Research 5-9", 4, 8),
        ("Needs Research 5-10", 4, 9),
        ("Needs Research 5-11", 4, 10),
        ("Needs Research 5-12", 4, 11),
        ("Needs Research 6-7", 5, 6),
        ("Needs Research 6-8", 5, 7),
        ("Needs Research 6-9", 5, 8),
        ("Needs Research 6-10", 5, 9),
        ("Needs Research 6-11", 5, 10),
        ("Needs Research 6-12", 5, 11),
        ("Needs Research 7-8", 6, 7),
        ("Needs Research 7-9", 6, 8),
        ("Needs Research 7-10", 6, 9),
        ("Needs Research 7-11", 6, 10),
        ("Needs Research 7-12", 6, 11),
        ("Needs Research 8-9", 7, 8),
        ("Needs Research 8-10", 7, 9),
        ("Needs Research 8-11", 7, 10),
        ("Needs Research 8-12", 7, 11),
        ("Needs Research 9-10", 8, 9),
        ("Needs Research 9-11", 8, 10),
        ("Needs Research 9-12", 8, 11),
        ("Needs Research 10-11", 9, 10),
        ("Needs Research 10-12", 9, 11),
        ("Needs Research 11-12", 10, 11),
    ];
}
