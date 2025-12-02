pub struct TriadVocabulary;

impl TriadVocabulary {
    pub const TERM_CHARACTERS: [&'static str; 3] = ["Will", "Function", "Being"];
    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 3] = [
        ("Act1", "Will", "Function"),
        ("Act2", "Function", "Being"),
        ("Act3", "Being", "Will"),
    ];
    pub const SOURCE: &'static str = "Elementary Systematics";
}
