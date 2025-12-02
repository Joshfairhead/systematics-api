pub struct PentadVocabulary;

impl PentadVocabulary {
    pub const TERM_CHARACTERS: [&'static str; 5] = ["Purpose", "Higher Potential", "Quintessence", "Lower Potential", "Source"];
    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 10] = [
        ("Range of Potential", "Higher Potential", "Lower Potential"),
        ("Range of Significance", "Purpose", "Source"),
        ("Aspiration", "Quintessence", "Higher Potential"),
        ("Operation", "Quintessence", "Lower Potential"),
        ("Output", "Higher Potential", "Purpose"),
        ("Input", "Lower Potential", "Source"),
        ("Qualitative Match", "Quintessence", "Purpose"),
        ("Quantitative Match", "Quintessence", "Source"),
        ("Form", "Lower Potential", "Purpose"),
        ("Function", "Higher Potential", "Source"),
    ];
}
