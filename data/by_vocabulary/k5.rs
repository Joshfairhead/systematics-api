pub struct PentadVocabulary;

impl PentadVocabulary {
    pub const TERMS: [&'static str; 5] = ["Purpose", "Higher Potential", "Quintessence", "Lower Potential", "Source"];
    pub const CONNECTIVES: [(&'static str, usize, usize); 10] = [
        ("Mutuality 1-2", 0, 1),
        ("Mutuality 1-3", 0, 2),
        ("Mutuality 1-4", 0, 3),
        ("Mutuality 1-5", 0, 4),
        ("Mutuality 2-3", 1, 2),
        ("Mutuality 2-4", 1, 3),
        ("Mutuality 2-5", 1, 4),
        ("Mutuality 3-4", 2, 3),
        ("Mutuality 3-5", 2, 4),
        ("Mutuality 4-5", 3, 4),
    ];
}
