pub struct TetradVocabulary;

impl TetradVocabulary {
    pub const TERM_CHARACTERS: [&'static str; 4] = ["Ideal", "Directive", "Instrumental", "Ground"];
    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 6] = [
        ("Receptive Regard", "Ideal", "Directive"),
        ("Effectual Compatibility", "Ideal", "Instrumental"),
        ("Motivational Imperative", "Ideal", "Ground"),
        ("Demonstrable Activity", "Directive", "Instrumental"),
        ("Material Mastery", "Directive", "Ground"),
        ("Technical Power", "Instrumental", "Ground"),
    ];
}
