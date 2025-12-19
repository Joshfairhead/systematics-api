/// Hyparchic Registry: The scale-free Color ↔ Index bidirectional mapping
///
/// This registry applies universally to both:
/// - Macro level: System-level identity (System 1-12)
/// - Micro level: Fiber-level identity (positions within systems)
///
/// The mapping is immutable and defines the fundamental identity relationship.

use std::fmt;

/// Index represents a one-based position (1-12) in the universal ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Index(u8);

impl Index {
    /// Create a new Index from a one-based value (1-12)
    pub fn new(value: u8) -> Result<Self, &'static str> {
        if (1..=12).contains(&value) {
            Ok(Index(value))
        } else {
            Err("Index must be between 1 and 12")
        }
    }

    /// Get the one-based value
    pub fn value(&self) -> u8 {
        self.0
    }

    /// Convert to zero-based for internal array access
    pub fn to_zero_based(&self) -> usize {
        (self.0 - 1) as usize
    }

    /// Create from zero-based array position
    pub fn from_zero_based(position: usize) -> Result<Self, &'static str> {
        if position < 12 {
            Ok(Index((position + 1) as u8))
        } else {
            Err("Position must be less than 12")
        }
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for Index {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Index::new(value)
    }
}

impl From<Index> for u8 {
    fn from(index: Index) -> u8 {
        index.0
    }
}

/// Color represents the twelve universal colors mapped to indices
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Red,          // 1
    Blue,         // 2
    Yellow,       // 3
    Green,        // 4
    Purple,       // 5
    Orange,       // 6
    LightBlue,    // 7
    Brown,        // 8
    Pink,         // 9
    White,        // 10
    GreySilver,   // 11
    BrightOrange, // 12 (Gold)
}

impl Color {
    /// Get the color name as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Color::Red => "Red",
            Color::Blue => "Blue",
            Color::Yellow => "Yellow",
            Color::Green => "Green",
            Color::Purple => "Purple",
            Color::Orange => "Orange",
            Color::LightBlue => "Light Blue",
            Color::Brown => "Brown",
            Color::Pink => "Pink",
            Color::White => "White",
            Color::GreySilver => "Grey/Silver",
            Color::BrightOrange => "Bright Orange/Gold",
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// The Hyparchic Registry: immutable bidirectional Color ↔ Index mapping
static HYPARCHIC_REGISTRY: [Color; 12] = [
    Color::Red,          // Index 1
    Color::Blue,         // Index 2
    Color::Yellow,       // Index 3
    Color::Green,        // Index 4
    Color::Purple,       // Index 5
    Color::Orange,       // Index 6
    Color::LightBlue,    // Index 7
    Color::Brown,        // Index 8
    Color::Pink,         // Index 9
    Color::White,        // Index 10
    Color::GreySilver,   // Index 11
    Color::BrightOrange, // Index 12
];

/// HyparchicRegistry provides bidirectional lookups between Color and Index
pub struct HyparchicRegistry;

impl HyparchicRegistry {
    /// Get color for a given index (Index → Color)
    pub fn get_color(index: Index) -> Color {
        HYPARCHIC_REGISTRY[index.to_zero_based()]
    }

    /// Get index for a given color (Color → Index)
    pub fn get_index(color: Color) -> Index {
        for (position, &registry_color) in HYPARCHIC_REGISTRY.iter().enumerate() {
            if registry_color == color {
                return Index::from_zero_based(position).unwrap();
            }
        }
        unreachable!("All colors must exist in registry")
    }

    /// Get all color-index pairs
    pub fn all_pairs() -> Vec<(Index, Color)> {
        HYPARCHIC_REGISTRY
            .iter()
            .enumerate()
            .map(|(pos, &color)| (Index::from_zero_based(pos).unwrap(), color))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_creation() {
        assert!(Index::new(1).is_ok());
        assert!(Index::new(12).is_ok());
        assert!(Index::new(0).is_err());
        assert!(Index::new(13).is_err());
    }

    #[test]
    fn test_index_conversions() {
        let index = Index::new(1).unwrap();
        assert_eq!(index.value(), 1);
        assert_eq!(index.to_zero_based(), 0);

        let index = Index::new(12).unwrap();
        assert_eq!(index.value(), 12);
        assert_eq!(index.to_zero_based(), 11);
    }

    #[test]
    fn test_bidirectional_mapping() {
        // Index → Color
        let index1 = Index::new(1).unwrap();
        assert_eq!(HyparchicRegistry::get_color(index1), Color::Red);

        let index2 = Index::new(2).unwrap();
        assert_eq!(HyparchicRegistry::get_color(index2), Color::Blue);

        let index3 = Index::new(3).unwrap();
        assert_eq!(HyparchicRegistry::get_color(index3), Color::Yellow);

        // Color → Index
        assert_eq!(HyparchicRegistry::get_index(Color::Red), Index::new(1).unwrap());
        assert_eq!(HyparchicRegistry::get_index(Color::Blue), Index::new(2).unwrap());
        assert_eq!(HyparchicRegistry::get_index(Color::Yellow), Index::new(3).unwrap());
    }

    #[test]
    fn test_all_colors_mapped() {
        let pairs = HyparchicRegistry::all_pairs();
        assert_eq!(pairs.len(), 12);

        // Verify specific mappings
        assert_eq!(pairs[0], (Index::new(1).unwrap(), Color::Red));
        assert_eq!(pairs[11], (Index::new(12).unwrap(), Color::BrightOrange));
    }

    #[test]
    fn test_commutative_property() {
        // For all indices, Index → Color → Index should return original
        for i in 1..=12 {
            let index = Index::new(i).unwrap();
            let color = HyparchicRegistry::get_color(index);
            let recovered_index = HyparchicRegistry::get_index(color);
            assert_eq!(index, recovered_index, "Commutative property failed for index {}", i);
        }
    }
}
