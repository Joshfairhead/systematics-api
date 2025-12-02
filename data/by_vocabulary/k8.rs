pub struct OctadVocabulary;

impl OctadVocabulary {
    pub const TERM_CHARACTERS: [&'static str; 8] = [
        "Smallest Significant Holon",
        "Critical Functions",
        "Supportive Platform",
        "Necessary Resourcing",
        "Integrative Totality",
        "Inherent Values",
        "Intrinsic Nature",
        "Organisational Modes"
    ];
    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 28] = [
        ("Component1", "Smallest Significant Holon", "Critical Functions"),
        ("Component2", "Smallest Significant Holon", "Supportive Platform"),
        ("Component3", "Smallest Significant Holon", "Necessary Resourcing"),
        ("Component4", "Smallest Significant Holon", "Integrative Totality"),
        ("Component5", "Smallest Significant Holon", "Inherent Values"),
        ("Component6", "Smallest Significant Holon", "Intrinsic Nature"),
        ("Component7", "Smallest Significant Holon", "Organisational Modes"),
        ("Component8", "Critical Functions", "Supportive Platform"),
        ("Component9", "Critical Functions", "Necessary Resourcing"),
        ("Component10", "Critical Functions", "Integrative Totality"),
        ("Component11", "Critical Functions", "Inherent Values"),
        ("Component12", "Critical Functions", "Intrinsic Nature"),
        ("Component13", "Critical Functions", "Organisational Modes"),
        ("Component14", "Supportive Platform", "Necessary Resourcing"),
        ("Component15", "Supportive Platform", "Integrative Totality"),
        ("Component16", "Supportive Platform", "Inherent Values"),
        ("Component17", "Supportive Platform", "Intrinsic Nature"),
        ("Component18", "Supportive Platform", "Organisational Modes"),
        ("Component19", "Necessary Resourcing", "Integrative Totality"),
        ("Component20", "Necessary Resourcing", "Inherent Values"),
        ("Component21", "Necessary Resourcing", "Intrinsic Nature"),
        ("Component22", "Necessary Resourcing", "Organisational Modes"),
        ("Component23", "Integrative Totality", "Inherent Values"),
        ("Component24", "Integrative Totality", "Intrinsic Nature"),
        ("Component25", "Integrative Totality", "Organisational Modes"),
        ("Component26", "Inherent Values", "Intrinsic Nature"),
        ("Component27", "Inherent Values", "Organisational Modes"),
        ("Component28", "Intrinsic Nature", "Organisational Modes"),
    ];
    pub const SOURCE: &'static str = "Qualsystems Book";
}
