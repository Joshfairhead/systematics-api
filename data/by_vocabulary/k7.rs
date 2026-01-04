pub struct HeptadVocabulary;

impl HeptadVocabulary {
    pub const TERMS: [&'static str; 7] = ["Insight", "Research", "Design", "Synthesis", "Application", "Delivery", "Value"];
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
