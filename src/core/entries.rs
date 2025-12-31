//! Entry types for the property graph.
//!
//! Each entry has structural info (order, position, type) embedded.
//! Entries are the nodes in our property graph.

use serde::{Deserialize, Serialize};

use super::language::Language;

/// 3D point for geometric coordinates
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

/// Character is the semantic content, independent of structural position.
/// Same Character can appear as a Term (at a position) or referenced by a Connective (as a link).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Character {
    pub id: String,
    /// The vocabulary language (Canonical, Energy, Values, Society)
    pub language: Language,
    /// The semantic value (e.g., "Will", "act1")
    pub value: String,
}

impl Character {
    pub fn new(id: impl Into<String>, language: Language, value: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            language,
            value: value.into(),
        }
    }

    /// Create a character with an auto-generated ID
    pub fn with_auto_id(language: Language, value: impl Into<String>) -> Self {
        let value = value.into();
        let id = format!(
            "char_{}_{}",
            language.to_string().to_lowercase(),
            value.to_lowercase().replace(' ', "_")
        );
        Self {
            id,
            language,
            value,
        }
    }
}

/// Term is a positional entry referencing a Character.
/// Terms exist at a specific order and position within a system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Term {
    pub id: String,
    /// System order (1-12)
    pub order: u8,
    /// Position within the system (1 to order)
    pub position: u8,
    /// ID of the Character entry this term references
    pub character: String,
}

impl Term {
    pub fn new(
        id: impl Into<String>,
        order: u8,
        position: u8,
        character: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            order,
            position,
            character: character.into(),
        }
    }

    /// Create a term with an auto-generated ID
    pub fn with_auto_id(order: u8, position: u8, character: impl Into<String>) -> Self {
        let character = character.into();
        Self {
            id: format!("term_{}_{}", order, position),
            order,
            position,
            character,
        }
    }
}

/// Coordinate represents a 3D point at a specific order and position.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coordinate {
    pub id: String,
    /// System order (1-12)
    pub order: u8,
    /// Position within the system (1 to order)
    pub position: u8,
    /// Universal hyparchic index (1-12) - defaults to position if not set
    pub index: Option<u8>,
    /// 3D coordinate value
    pub value: Point3d,
}

impl Coordinate {
    pub fn new(id: impl Into<String>, order: u8, position: u8, value: Point3d) -> Self {
        Self {
            id: id.into(),
            order,
            position,
            index: None,
            value,
        }
    }

    /// Create a coordinate with an auto-generated ID
    pub fn with_auto_id(order: u8, position: u8, value: Point3d) -> Self {
        Self {
            id: format!("coord_{}_{}", order, position),
            order,
            position,
            index: None,
            value,
        }
    }

    /// Create a coordinate with a specific universal index
    pub fn with_index(order: u8, position: u8, index: u8, value: Point3d) -> Self {
        Self {
            id: format!("coord_{}_{}", order, position),
            order,
            position,
            index: Some(index),
            value,
        }
    }
}

/// Colour represents a color value at a specific order and position.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Colour {
    pub id: String,
    /// System order (for system-level colour, e.g., Triad=Yellow)
    pub order: u8,
    /// Position within the system (for position-specific colours)
    pub position: u8,
    /// Universal hyparchic index (1-12) - defaults to position if not set
    pub index: Option<u8>,
    /// Representation language (Hex or Name)
    pub language: Language,
    /// The color value (e.g., "#FF0000" or "Red")
    pub value: String,
}

impl Colour {
    pub fn new(
        id: impl Into<String>,
        order: u8,
        position: u8,
        language: Language,
        value: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            order,
            position,
            index: None,
            language,
            value: value.into(),
        }
    }

    /// Create a colour with an auto-generated ID
    pub fn with_auto_id(
        order: u8,
        position: u8,
        language: Language,
        value: impl Into<String>,
    ) -> Self {
        Self {
            id: format!("colour_{}_{}_{}", order, position, language.to_string().to_lowercase()),
            order,
            position,
            index: None,
            language,
            value: value.into(),
        }
    }

    /// Create a colour with a specific universal index
    pub fn with_index(
        order: u8,
        position: u8,
        index: u8,
        language: Language,
        value: impl Into<String>,
    ) -> Self {
        Self {
            id: format!("colour_{}_{}_{}", order, position, language.to_string().to_lowercase()),
            order,
            position,
            index: Some(index),
            language,
            value: value.into(),
        }
    }
}

/// TermDesignation is a per-order label that applies to all terms in a system.
/// For example, Order 3 terms are called "Impulses".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TermDesignation {
    pub id: String,
    /// System order (1-12)
    pub order: u8,
    /// The designation value (e.g., "Impulses", "Sources", "Limits")
    pub value: String,
}

impl TermDesignation {
    pub fn new(id: impl Into<String>, order: u8, value: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            order,
            value: value.into(),
        }
    }

    /// Create a term designation with an auto-generated ID
    pub fn with_auto_id(order: u8, value: impl Into<String>) -> Self {
        Self {
            id: format!("term_des_{}", order),
            order,
            value: value.into(),
        }
    }
}

/// ConnectiveDesignation is a per-order label that applies to all connectives in a system.
/// For example, Order 3 connectives are called "Acts".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectiveDesignation {
    pub id: String,
    /// System order (1-12)
    pub order: u8,
    /// The designation value (e.g., "Acts", "Interplays", "Steps")
    pub value: String,
}

impl ConnectiveDesignation {
    pub fn new(id: impl Into<String>, order: u8, value: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            order,
            value: value.into(),
        }
    }

    /// Create a connective designation with an auto-generated ID
    pub fn with_auto_id(order: u8, value: impl Into<String>) -> Self {
        Self {
            id: format!("conn_des_{}", order),
            order,
            value: value.into(),
        }
    }
}

/// CoherenceAttribute is a per-order attribute describing the coherence quality.
/// For example, Order 3 has coherence "Dynamism".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoherenceAttribute {
    pub id: String,
    /// System order (1-12)
    pub order: u8,
    /// The coherence value (e.g., "Dynamism")
    pub value: String,
}

impl CoherenceAttribute {
    pub fn new(id: impl Into<String>, order: u8, value: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            order,
            value: value.into(),
        }
    }

    /// Create a coherence attribute with an auto-generated ID
    pub fn with_auto_id(order: u8, value: impl Into<String>) -> Self {
        Self {
            id: format!("coherence_{}", order),
            order,
            value: value.into(),
        }
    }
}

/// SystemName provides the human-readable name for a system order.
/// For example, Order 3 is "Triad".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemName {
    pub id: String,
    /// System order (1-12)
    pub order: u8,
    /// The system name (e.g., "Monad", "Dyad", "Triad")
    pub value: String,
}

impl SystemName {
    pub fn new(id: impl Into<String>, order: u8, value: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            order,
            value: value.into(),
        }
    }

    /// Create a system name with an auto-generated ID
    pub fn with_auto_id(order: u8, value: impl Into<String>) -> Self {
        Self {
            id: format!("system_{}", order),
            order,
            value: value.into(),
        }
    }

    /// Get the standard name for a given order
    pub fn standard_name(order: u8) -> Option<&'static str> {
        match order {
            1 => Some("Monad"),
            2 => Some("Dyad"),
            3 => Some("Triad"),
            4 => Some("Tetrad"),
            5 => Some("Pentad"),
            6 => Some("Hexad"),
            7 => Some("Heptad"),
            8 => Some("Octad"),
            9 => Some("Ennead"),
            10 => Some("Decad"),
            11 => Some("Undecad"),
            12 => Some("Dodecad"),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_creation() {
        let char = Character::with_auto_id(Language::Canonical, "Will");
        assert_eq!(char.id, "char_canonical_will");
        assert_eq!(char.value, "Will");
        assert_eq!(char.language, Language::Canonical);
    }

    #[test]
    fn test_term_creation() {
        let term = Term::with_auto_id(3, 1, "char_will");
        assert_eq!(term.id, "term_3_1");
        assert_eq!(term.order, 3);
        assert_eq!(term.position, 1);
    }

    #[test]
    fn test_system_name_standard() {
        assert_eq!(SystemName::standard_name(1), Some("Monad"));
        assert_eq!(SystemName::standard_name(3), Some("Triad"));
        assert_eq!(SystemName::standard_name(12), Some("Dodecad"));
        assert_eq!(SystemName::standard_name(13), None);
    }

    #[test]
    fn test_point3d() {
        let point = Point3d::new(1.0, 0.0, 0.0);
        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, 0.0);
        assert_eq!(point.z, 0.0);
    }
}
