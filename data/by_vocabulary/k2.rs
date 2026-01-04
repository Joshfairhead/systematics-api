pub struct DyadVocabulary;

impl DyadVocabulary {
    pub const TERMS: [&'static str; 2] = ["Essence", "Existence"];
    pub const CONNECTIVES: [(&'static str, usize, usize); 1] = [
        ("Force", 0, 1),
    ];
}
