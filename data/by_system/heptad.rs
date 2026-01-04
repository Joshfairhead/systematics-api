pub struct HeptadSystem;

impl HeptadSystem {
    // Metadata (homotopy types)
    pub const SYSTEM_NAME: &'static str = "Heptad";
    pub const COHERENCE: &'static str = "Generation";
    pub const TERM_DESIGNATION: &'static str = "States";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Intervals";

    // Parallel arrays - bimorphic by index (index 0 = position 1, etc.)
    // Position 1=Insight, 2=Application, 3=Design, 4=Research, 5=Synthesis, 6=Delivery, 7=Value
    pub const TERMS: [&'static str; 7] = ["Insight", "Application", "Design", "Research", "Synthesis", "Delivery", "Value"];
    pub const COLOURS_HEX: [&'static str; 7] = ["#FF0000", "#0000FF", "#FFFF00", "#099902", "#9900FF", "#FFA500", "#00FFFF"];
    pub const COLOURS_NAME: [&'static str; 7] = ["Red", "Blue", "Yellow", "Green", "Purple", "Orange", "Light Blue"];
    pub const COORDS: [(f64, f64); 7] = [
        (0.0, 1.0),       // Insight (12 o'clock)
        (-0.43, -0.90),   // Application
        (0.97, -0.22),    // Design
        (0.78, 0.62),     // Research
        (0.43, -0.90),    // Synthesis
        (-0.97, -0.22),   // Delivery
        (-0.78, 0.62),    // Value
    ];

    // Connectives - reference by position INDEX (complete graph: 21 edges)
    // Semantic names need research for heptad
    pub const CONNECTIVES: [(&'static str, usize, usize); 21] = [
        ("Interval 1-2", 0, 1),
        ("Interval 1-3", 0, 2),
        ("Interval 1-4", 0, 3),
        ("Interval 1-5", 0, 4),
        ("Interval 1-6", 0, 5),
        ("Interval 1-7", 0, 6),
        ("Interval 2-3", 1, 2),
        ("Interval 2-4", 1, 3),
        ("Interval 2-5", 1, 4),
        ("Interval 2-6", 1, 5),
        ("Interval 2-7", 1, 6),
        ("Interval 3-4", 2, 3),
        ("Interval 3-5", 2, 4),
        ("Interval 3-6", 2, 5),
        ("Interval 3-7", 2, 6),
        ("Interval 4-5", 3, 4),
        ("Interval 4-6", 3, 5),
        ("Interval 4-7", 3, 6),
        ("Interval 5-6", 4, 5),
        ("Interval 5-7", 4, 6),
        ("Interval 6-7", 5, 6),
    ];
}
