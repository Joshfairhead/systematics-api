use crate::core::Index;

/// Essence data (metadata) for each system order
#[derive(Debug, Clone)]
pub struct EssenceData {
    pub system_name: &'static str,
    pub coherence: &'static str,
    pub term_designation: Option<&'static str>,
    pub connective_designation: Option<&'static str>,
}

/// Static registry of Essence data indexed by Index (1-12)
static ESSENCE_REGISTRY: [EssenceData; 12] = [
    // Index 1: Monad
    EssenceData {
        system_name: "Monad",
        coherence: "Unity",
        term_designation: Some("Totality"),
        connective_designation: Some("Unity"),
    },
    // Index 2: Dyad
    EssenceData {
        system_name: "Dyad",
        coherence: "Complementarity",
        term_designation: Some("Poles"),
        connective_designation: Some("Force"),
    },
    // Index 3: Triad
    EssenceData {
        system_name: "Triad",
        coherence: "Dynamism",
        term_designation: Some("Impulses"),
        connective_designation: Some("Acts"),
    },
    // Index 4: Tetrad
    EssenceData {
        system_name: "Tetrad",
        coherence: "Activity Field",
        term_designation: Some("Sources"),
        connective_designation: Some("Orders"),
    },
    // Index 5: Pentad
    EssenceData {
        system_name: "Pentad",
        coherence: "Significance & Potential",
        term_designation: Some("Limits"),
        connective_designation: Some("Reciprocals"),
    },
    // Index 6: Hexad
    EssenceData {
        system_name: "Hexad",
        coherence: "Coalescence",
        term_designation: Some("Laws"),
        connective_designation: Some("Steps"),
    },
    // Index 7: Heptad
    EssenceData {
        system_name: "Heptad",
        coherence: "Generation",
        term_designation: Some("Characters"),
        connective_designation: Some("Phases"),
    },
    // Index 8: Octad
    EssenceData {
        system_name: "Octad",
        coherence: "Self Sufficiency",
        term_designation: Some("Elements"),
        connective_designation: Some("Components"),
    },
    // Index 9: Ennead
    EssenceData {
        system_name: "Ennead",
        coherence: "Transformation",
        term_designation: None,
        connective_designation: None,
    },
    // Index 10: Decad
    EssenceData {
        system_name: "Decad",
        coherence: "Intrinsic Harmony",
        term_designation: None,
        connective_designation: None,
    },
    // Index 11: Undecad
    EssenceData {
        system_name: "Undecad",
        coherence: "Articulate Symmetry",
        term_designation: None,
        connective_designation: None,
    },
    // Index 12: Dodecad
    EssenceData {
        system_name: "Dodecad",
        coherence: "Perfection",
        term_designation: None,
        connective_designation: None,
    },
];

/// The Essence Layer provides static lookups of universal data
pub struct Essence;

impl Essence {
    /// Look up essence data by Index
    pub fn lookup(index: Index) -> &'static EssenceData {
        &ESSENCE_REGISTRY[index.to_zero_based()]
    }

    /// Get system name for an index
    pub fn system_name(index: Index) -> &'static str {
        Self::lookup(index).system_name
    }

    /// Get coherence attribute for an index
    pub fn coherence(index: Index) -> &'static str {
        Self::lookup(index).coherence
    }

    /// Get term designation for an index (if available)
    pub fn term_designation(index: Index) -> Option<&'static str> {
        Self::lookup(index).term_designation
    }

    /// Get connective designation for an index (if available)
    pub fn connective_designation(index: Index) -> Option<&'static str> {
        Self::lookup(index).connective_designation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_essence_lookup() {
        let monad = Index::One;
        assert_eq!(Essence::system_name(monad), "Monad");
        assert_eq!(Essence::coherence(monad), "Unity");
        assert_eq!(Essence::term_designation(monad), Some("Totality"));
        assert_eq!(Essence::connective_designation(monad), Some("Unity"));

        let dodecad = Index::Twelve;
        assert_eq!(Essence::system_name(dodecad), "Dodecad");
        assert_eq!(Essence::coherence(dodecad), "Perfection");
        assert_eq!(Essence::term_designation(dodecad), None);
        assert_eq!(Essence::connective_designation(dodecad), None);
    }
}
