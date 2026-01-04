pub struct TetradVocabulary;

impl TetradVocabulary {
    pub const TERMS: [&'static str; 4] = ["Ideal", "Directive", "Instrumental", "Ground"];
    pub const CONNECTIVES: [(&'static str, usize, usize); 6] = [
        ("Receptive Regard", 0, 1),
        ("Effectual Compatibility", 0, 2),
        ("Motivational Imperative", 0, 3),
        ("Demonstrable Activity", 1, 2),
        ("Material Mastery", 1, 3),
        ("Technical Power", 2, 3),
    ];
}
