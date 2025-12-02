pub struct HexadVocabulary;

impl HexadVocabulary {
    pub const TERM_CHARACTERS: [&'static str; 6] = ["Resources", "Values", "Options", "Criteria", "Facts", "Priorities"];
    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 15] = [
        ("Step1", "Resources", "Values"),
        ("Step2", "Resources", "Options"),
        ("Step3", "Resources", "Criteria"),
        ("Step4", "Resources", "Facts"),
        ("Step5", "Resources", "Priorities"),
        ("Step6", "Values", "Options"),
        ("Step7", "Values", "Criteria"),
        ("Step8", "Values", "Facts"),
        ("Step9", "Values", "Priorities"),
        ("Step10", "Options", "Criteria"),
        ("Step11", "Options", "Facts"),
        ("Step12", "Options", "Priorities"),
        ("Step13", "Criteria", "Facts"),
        ("Step14", "Criteria", "Priorities"),
        ("Step15", "Facts", "Priorities"),
    ];
}
