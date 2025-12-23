//! Registry: The central lookup for all Systematics data.
//!
//! The Registry is the single source of truth for looking up
//! content values from reference types. It supports:
//! - Static canonical defaults (compile-time)
//! - Runtime overrides for alternative vocabularies/schemes
//!
//! All content types (Node, Point, Term, Color, etc.) use the Registry
//! to look up their actual values.

use crate::core::Index;
use crate::core::Color;

/// Static canonical color names (default reference)
const CANONICAL_COLOR_NAMES: [&str; 12] = [
    "Red",           // 1
    "Blue",          // 2
    "Yellow",        // 3
    "Green",         // 4
    "Purple",        // 5
    "Orange",        // 6
    "Light Blue",    // 7
    "Brown",         // 8
    "Pink",          // 9
    "White",         // 10
    "Grey/Silver",   // 11
    "Bright Orange", // 12 (Gold)
];

/// Static canonical color hex codes (default reference)
const CANONICAL_COLOR_HEX: [&str; 12] = [
    "#FF0000", // 1 - Red
    "#0000FF", // 2 - Blue
    "#E6E600", // 3 - Yellow (slightly darker for contrast)
    "#00CC00", // 4 - Green (slightly darker for contrast)
    "#9370DB", // 5 - Purple (medium purple)
    "#FFA500", // 6 - Orange
    "#00BFFF", // 7 - Light Blue (Deep Sky Blue)
    "#A0522D", // 8 - Brown (Sienna)
    "#FF1493", // 9 - Pink (Deep Pink)
    "#FFFFFF", // 10 - White
    "#C0C0C0", // 11 - Grey/Silver
    "#FFD700", // 12 - Gold
];

/// Static canonical system names
const CANONICAL_SYSTEM_NAMES: [&str; 12] = [
    "Monad",
    "Dyad",
    "Triad",
    "Tetrad",
    "Pentad",
    "Hexad",
    "Heptad",
    "Octad",
    "Ennead",
    "Decad",
    "Undecad",
    "Dodecad",
];

/// Static canonical coherence attributes
const CANONICAL_COHERENCE: [&str; 12] = [
    "Unity",              // 1
    "Complementarity",    // 2
    "Dynamism",           // 3
    "Activity Field",     // 4
    "Significance",       // 5
    "Coalescence",        // 6
    "Generation",         // 7
    "Self-Sufficiency",   // 8
    "Transformation",     // 9
    "Intrinsic Harmony",  // 10
    "Articulate Symmetry",// 11
    "Harmony",            // 12
];

/// Static canonical term designations (Some systems don't have established designations)
const CANONICAL_TERM_DESIGNATIONS: [Option<&str>; 12] = [
    Some("Totality"),  // 1
    Some("Poles"),     // 2
    Some("Impulses"),  // 3
    Some("Sources"),   // 4
    Some("Limits"),    // 5
    Some("Laws"),      // 6
    Some("States"),    // 7
    Some("Elements"),  // 8
    None,              // 9
    None,              // 10
    None,              // 11
    None,              // 12
];

/// Static canonical connective designations
const CANONICAL_CONNECTIVE_DESIGNATIONS: [Option<&str>; 12] = [
    Some("Unity"),        // 1 (connectionless)
    Some("Force"),        // 2
    Some("Acts"),         // 3
    Some("Interplays"),   // 4
    Some("Mutualities"),  // 5
    Some("Steps"),        // 6
    Some("Intervals"),    // 7
    Some("Components"),   // 8
    None,                 // 9
    None,                 // 10
    None,                 // 11
    None,                 // 12
];

/// The Registry is the central lookup for all Systematics data.
///
/// It provides static canonical defaults and will support runtime
/// overrides for alternative vocabularies/schemes in future iterations.
#[derive(Debug, Clone)]
pub struct Registry {
    // Future: runtime overrides would be stored here
    // color_scheme: Option<String>,
    // vocabulary: Option<String>,
}

impl Registry {
    /// Create a new Registry with canonical defaults
    pub fn new() -> Self {
        Self {}
    }

    /// Create a canonical (default) registry
    pub fn canonical() -> Self {
        Self::new()
    }

    // ========== System Lookups ==========

    /// Get the system name for an index
    pub fn system_name(&self, index: Index) -> &'static str {
        CANONICAL_SYSTEM_NAMES[index.to_zero_based()]
    }

    /// Get the coherence attribute for an index
    pub fn coherence(&self, index: Index) -> &'static str {
        CANONICAL_COHERENCE[index.to_zero_based()]
    }

    /// Get the term designation for an index (if available)
    pub fn term_designation(&self, index: Index) -> Option<&'static str> {
        CANONICAL_TERM_DESIGNATIONS[index.to_zero_based()]
    }

    /// Get the connective designation for an index (if available)
    pub fn connective_designation(&self, index: Index) -> Option<&'static str> {
        CANONICAL_CONNECTIVE_DESIGNATIONS[index.to_zero_based()]
    }

    // ========== Color Lookups ==========

    /// Get the color name for an index (optionally with a reference/scheme)
    pub fn color_name(&self, index: Index, _reference: Option<&str>) -> &'static str {
        // Future: check for runtime override based on reference
        CANONICAL_COLOR_NAMES[index.to_zero_based()]
    }

    /// Get the color hex code for an index (optionally with a reference/scheme)
    pub fn color_hex(&self, index: Index, _reference: Option<&str>) -> &'static str {
        // Future: check for runtime override based on reference
        CANONICAL_COLOR_HEX[index.to_zero_based()]
    }

    /// Get the color for an index
    pub fn color(&self, index: Index) -> Color {
        Color::from_index(index)
    }

    // ========== Index Lookups ==========

    /// Get the index for a color
    pub fn index(&self, color: Color) -> Index {
        color.to_index()
    }

    // ========== Future: Runtime Configuration ==========

    // pub fn set_vocabulary(&mut self, name: &str, data: VocabularyData) { ... }
    // pub fn set_color_scheme(&mut self, name: &str, data: ColorSchemeData) { ... }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global canonical registry for convenient access
/// Use Registry::new() if you need a mutable registry for runtime configuration
pub static CANONICAL: std::sync::LazyLock<Registry> =
    std::sync::LazyLock::new(Registry::canonical);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_names() {
        let registry = Registry::new();
        assert_eq!(registry.system_name(Index::One), "Monad");
        assert_eq!(registry.system_name(Index::Three), "Triad");
        assert_eq!(registry.system_name(Index::Twelve), "Dodecad");
    }

    #[test]
    fn test_coherence() {
        let registry = Registry::new();
        assert_eq!(registry.coherence(Index::One), "Unity");
        assert_eq!(registry.coherence(Index::Three), "Dynamism");
    }

    #[test]
    fn test_color_names() {
        let registry = Registry::new();
        assert_eq!(registry.color_name(Index::One, None), "Red");
        assert_eq!(registry.color_name(Index::Two, None), "Blue");
        assert_eq!(registry.color_name(Index::Three, None), "Yellow");
    }

    #[test]
    fn test_color_hex() {
        let registry = Registry::new();
        assert_eq!(registry.color_hex(Index::One, None), "#FF0000");
        assert_eq!(registry.color_hex(Index::Two, None), "#0000FF");
    }

    #[test]
    fn test_term_designations() {
        let registry = Registry::new();
        assert_eq!(registry.term_designation(Index::One), Some("Totality"));
        assert_eq!(registry.term_designation(Index::Three), Some("Impulses"));
        assert_eq!(registry.term_designation(Index::Nine), None);
    }

    #[test]
    fn test_connective_designations() {
        let registry = Registry::new();
        assert_eq!(registry.connective_designation(Index::Three), Some("Acts"));
        assert_eq!(registry.connective_designation(Index::Ten), None);
    }

    #[test]
    fn test_color_index_bijection() {
        let registry = Registry::new();
        for idx in Index::all() {
            let color = registry.color(idx);
            let recovered = registry.index(color);
            assert_eq!(idx, recovered);
        }
    }
}
