pub struct DyadVocabulary;

impl DyadVocabulary {
    pub const TERM_CHARACTERS: [&'static str; 2] = ["Essence", "Existence"];
    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 1] = [
        ("Force1", "Essence", "Existence"),
    ];
    pub const SOURCE: &'static str = "Elementary Systematics";
}
