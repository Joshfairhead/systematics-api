//! Term characters represent the specific manifestations of terms
//! within a system. Unlike term designations which are fixed categories,
//! characters can vary based on the specific system being modeled.

/// Characters for the Monadic system
#[derive(Debug, Clone, PartialEq)]
pub enum MonadicTermCharacters {
    /// The single term character for a Monad
    Unity,
}

/// Characters for the Dyadic system
#[derive(Debug, Clone, PartialEq)]
pub enum DyadicTermCharacters {
    /// First pole character
    Essence,
    /// Second pole character
    Existence,
}

/// Characters for the Triadic system
#[derive(Debug, Clone, PartialEq)]
pub enum TriadicTermCharacters {
    /// Position 1 - the affirming force
    Will,
    /// Position 2 - the denying force
    Function,
    /// Position 3 - the reconciling force
    Being,
}

/// Characters for the Tetradic system
#[derive(Debug, Clone, PartialEq)]
pub enum TetradicTermCharacters {
    /// First source character
    Ideal,
    /// Second source character
    Directive,
    /// Third source character
    Instrumental,
    /// Fourth source character
    Ground,
}

/// Characters for the Pentadic system
#[derive(Debug, Clone, PartialEq)]
pub enum PentadicTermCharacters {
    /// First limit character
    Purpose,
    /// Second limit character
    HigherPotential,
    /// Third limit character
    Quintessence,
    /// Fourth limit character
    LowerPotential,
    /// Fifth limit character
    Source,
}

/// Characters for the Hexadic system
#[derive(Debug, Clone, PartialEq)]
pub enum HexadicTermCharacters {
    /// First law character
    Resources,
    /// Second law character
    Values,
    /// Third law character
    Options,
    /// Fourth law character
    Criteria,
    /// Fifth law character
    Facts,
    /// Sixth law character
    Priorities,
}

/// Characters for the Heptadic system
#[derive(Debug, Clone, PartialEq)]
pub enum HeptadicTermCharacters {
    /// First state character
    Insight,
    /// Second state character
    Research,
    /// Third state character
    Design,
    /// Fourth state character
    Synthesis,
    /// Fifth state character
    Application,
    /// Sixth state character
    Delivery,
    /// Seventh state character
    Value,
}

/// Characters for the Octadic system
#[derive(Debug, Clone, PartialEq)]
pub enum OctadicTermCharacters {
    /// Position 1 element character
    InherentValues,
    /// Position 2 element character
    CriticalFunctions,
    /// Position 3 element character
    OrganisationalModes,
    /// Position 4 element character
    NecessaryResourcing,
    /// Position 5 element character
    IntrinsicNature,
    /// Position 6 element character
    SmallestSignificantHolon,
    /// Position 7 element character
    IntegrativeTotality,
    /// Position 8 element character
    SupportivePlatform,
}

/// A term character represents how a term manifests in a specific system
#[derive(Debug, Clone, PartialEq)]
pub enum TermCharacters {
    /// Monadic character
    Monadic(MonadicTermCharacters),
    /// Dyadic character
    Dyadic(DyadicTermCharacters),
    /// Triadic character
    Triadic(TriadicTermCharacters),
    /// Tetradic character
    Tetradic(TetradicTermCharacters),
    /// Pentadic character
    Pentadic(PentadicTermCharacters),
    /// Hexadic character
    Hexadic(HexadicTermCharacters),
    /// Heptadic character
    Heptadic(HeptadicTermCharacters),
    /// Octadic character
    Octadic(OctadicTermCharacters),
    /// Custom character (for systems 9-12 or custom implementations)
    Custom(String),
}

impl TermCharacters {
    /// Get the string representation of this character
    pub fn as_str(&self) -> String {
        match self {
            TermCharacters::Monadic(c) => match c {
                MonadicTermCharacters::Unity => "Unity".to_string(),
            },
            TermCharacters::Dyadic(c) => match c {
                DyadicTermCharacters::Essence => "Essence".to_string(),
                DyadicTermCharacters::Existence => "Existence".to_string(),
            },
            TermCharacters::Triadic(c) => match c {
                TriadicTermCharacters::Will => "Will".to_string(),
                TriadicTermCharacters::Function => "Function".to_string(),
                TriadicTermCharacters::Being => "Being".to_string(),
            },
            TermCharacters::Tetradic(c) => match c {
                TetradicTermCharacters::Ideal => "Ideal".to_string(),
                TetradicTermCharacters::Directive => "Directive".to_string(),
                TetradicTermCharacters::Instrumental => "Instrumental".to_string(),
                TetradicTermCharacters::Ground => "Ground".to_string(),
            },
            TermCharacters::Pentadic(c) => match c {
                PentadicTermCharacters::Purpose => "Purpose".to_string(),
                PentadicTermCharacters::HigherPotential => "Higher Potential".to_string(),
                PentadicTermCharacters::Quintessence => "Quintessence".to_string(),
                PentadicTermCharacters::LowerPotential => "Lower Potential".to_string(),
                PentadicTermCharacters::Source => "Source".to_string(),
            },
            TermCharacters::Hexadic(c) => match c {
                HexadicTermCharacters::Resources => "Resources".to_string(),
                HexadicTermCharacters::Values => "Values".to_string(),
                HexadicTermCharacters::Options => "Options".to_string(),
                HexadicTermCharacters::Criteria => "Criteria".to_string(),
                HexadicTermCharacters::Facts => "Facts".to_string(),
                HexadicTermCharacters::Priorities => "Priorities".to_string(),
            },
            TermCharacters::Heptadic(c) => match c {
                HeptadicTermCharacters::Insight => "Insight".to_string(),
                HeptadicTermCharacters::Research => "Research".to_string(),
                HeptadicTermCharacters::Design => "Design".to_string(),
                HeptadicTermCharacters::Synthesis => "Synthesis".to_string(),
                HeptadicTermCharacters::Application => "Application".to_string(),
                HeptadicTermCharacters::Delivery => "Delivery".to_string(),
                HeptadicTermCharacters::Value => "Value".to_string(),
            },
            TermCharacters::Octadic(c) => match c {
                OctadicTermCharacters::InherentValues => "Inherent Values".to_string(),
                OctadicTermCharacters::CriticalFunctions => "Critical Functions".to_string(),
                OctadicTermCharacters::OrganisationalModes => "Organisational Modes".to_string(),
                OctadicTermCharacters::NecessaryResourcing => "Necessary Resourcing".to_string(),
                OctadicTermCharacters::IntrinsicNature => "Intrinsic Nature".to_string(),
                OctadicTermCharacters::SmallestSignificantHolon => "Smallest Significant Holon".to_string(),
                OctadicTermCharacters::IntegrativeTotality => "Integrative Totality".to_string(),
                OctadicTermCharacters::SupportivePlatform => "Supportive Platform".to_string(),
            },
            TermCharacters::Custom(s) => s.clone(),
        }
    }
} 