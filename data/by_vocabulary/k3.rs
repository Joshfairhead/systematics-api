pub struct TriadVocabulary;

impl TriadVocabulary {
    pub const TERMS: [&'static str; 3] = ["Will", "Function", "Being"];
    pub const CONNECTIVES: [(&'static str, usize, usize); 3] = [
        ("Act 1-2", 0, 1),
        ("Act 2-3", 1, 2),
        ("Act 3-1", 2, 0),
    ];
}
