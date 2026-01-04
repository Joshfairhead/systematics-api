pub struct OctadVocabulary;

impl OctadVocabulary {
    pub const TERMS: [&'static str; 8] = [
        "Inherent Values",
        "Critical Functions",
        "Organisational Modes",
        "Necessary Resourcing",
        "Intrinsic Nature",
        "Smallest Significant Holon",
        "Integrative Totality",
        "Supportive Platform",
    ];
    pub const CONNECTIVES: [(&'static str, usize, usize); 28] = [
        ("Component 1-2", 0, 1),
        ("Component 1-3", 0, 2),
        ("Component 1-4", 0, 3),
        ("Component 1-5", 0, 4),
        ("Component 1-6", 0, 5),
        ("Component 1-7", 0, 6),
        ("Component 1-8", 0, 7),
        ("Component 2-3", 1, 2),
        ("Component 2-4", 1, 3),
        ("Component 2-5", 1, 4),
        ("Component 2-6", 1, 5),
        ("Component 2-7", 1, 6),
        ("Component 2-8", 1, 7),
        ("Component 3-4", 2, 3),
        ("Component 3-5", 2, 4),
        ("Component 3-6", 2, 5),
        ("Component 3-7", 2, 6),
        ("Component 3-8", 2, 7),
        ("Component 4-5", 3, 4),
        ("Component 4-6", 3, 5),
        ("Component 4-7", 3, 6),
        ("Component 4-8", 3, 7),
        ("Component 5-6", 4, 5),
        ("Component 5-7", 4, 6),
        ("Component 5-8", 4, 7),
        ("Component 6-7", 5, 6),
        ("Component 6-8", 5, 7),
        ("Component 7-8", 6, 7),
    ];
}
