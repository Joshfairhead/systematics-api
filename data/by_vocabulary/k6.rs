pub struct HexadVocabulary;

impl HexadVocabulary {
    pub const TERMS: [&'static str; 6] = ["Resources", "Values", "Options", "Criteria", "Facts", "Priorities"];
    pub const CONNECTIVES: [(&'static str, usize, usize); 15] = [
        ("Step 1-2", 0, 1),
        ("Step 1-3", 0, 2),
        ("Step 1-4", 0, 3),
        ("Step 1-5", 0, 4),
        ("Step 1-6", 0, 5),
        ("Step 2-3", 1, 2),
        ("Step 2-4", 1, 3),
        ("Step 2-5", 1, 4),
        ("Step 2-6", 1, 5),
        ("Step 3-4", 2, 3),
        ("Step 3-5", 2, 4),
        ("Step 3-6", 2, 5),
        ("Step 4-5", 3, 4),
        ("Step 4-6", 3, 5),
        ("Step 5-6", 4, 5),
    ];
}
