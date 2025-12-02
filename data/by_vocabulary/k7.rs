pub struct HeptadVocabulary;

impl HeptadVocabulary {
    pub const TERM_CHARACTERS: [&'static str; 7] = ["Insight", "Research", "Design", "Synthesis", "Application", "Delivery", "Value"];
    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 21] = [
        ("Needs Research", "Insight", "Research"),
        ("Needs Research", "Insight", "Design"),
        ("Needs Research", "Insight", "Synthesis"),
        ("Needs Research", "Insight", "Application"),
        ("Needs Research", "Insight", "Delivery"),
        ("Needs Research", "Insight", "Value"),
        ("Needs Research", "Research", "Design"),
        ("Needs Research", "Research", "Synthesis"),
        ("Needs Research", "Research", "Application"),
        ("Needs Research", "Research", "Delivery"),
        ("Needs Research", "Research", "Value"),
        ("Needs Research", "Design", "Synthesis"),
        ("Needs Research", "Design", "Application"),
        ("Needs Research", "Design", "Delivery"),
        ("Needs Research", "Design", "Value"),
        ("Needs Research", "Synthesis", "Application"),
        ("Needs Research", "Synthesis", "Delivery"),
        ("Needs Research", "Synthesis", "Value"),
        ("Needs Research", "Application", "Delivery"),
        ("Needs Research", "Application", "Value"),
        ("Needs Research", "Delivery", "Value"),
    ];
    pub const SOURCE: &'static str = "Elementary Systematics";
}
