//! Color: The symbolic reference type for Systematics.
//!
//! Color represents positions 1-12 in the universal ordering,
//! isomorphic with Index. The actual color names and hex codes
//! are looked up from the Registry.

use std::fmt;
use crate::core::Index;

/// Color represents a symbolic position (1-12) in the universal ordering.
///
/// This is isomorphic with Index - they identify the same positions.
/// Actual color names ("Red", "Blue", etc.) and hex codes are looked up
/// from the Registry, allowing for alternative color schemes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Color {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Eleven = 11,
    Twelve = 12,
}

impl Color {
    /// Get the numeric value (1-12)
    #[inline]
    pub fn value(&self) -> u8 {
        *self as u8
    }

    /// Create a Color from a numeric value (1-12)
    /// Returns None if the value is out of range
    pub fn from_value(n: u8) -> Option<Self> {
        match n {
            1 => Some(Color::One),
            2 => Some(Color::Two),
            3 => Some(Color::Three),
            4 => Some(Color::Four),
            5 => Some(Color::Five),
            6 => Some(Color::Six),
            7 => Some(Color::Seven),
            8 => Some(Color::Eight),
            9 => Some(Color::Nine),
            10 => Some(Color::Ten),
            11 => Some(Color::Eleven),
            12 => Some(Color::Twelve),
            _ => None,
        }
    }

    /// Convert to zero-based index for internal array access
    #[inline]
    pub fn to_zero_based(&self) -> usize {
        (self.value() - 1) as usize
    }

    /// Create from zero-based array position
    pub fn from_zero_based(position: usize) -> Option<Self> {
        if position < 12 {
            Self::from_value((position + 1) as u8)
        } else {
            None
        }
    }

    /// Get all colors in order
    pub fn all() -> [Color; 12] {
        [
            Color::One,
            Color::Two,
            Color::Three,
            Color::Four,
            Color::Five,
            Color::Six,
            Color::Seven,
            Color::Eight,
            Color::Nine,
            Color::Ten,
            Color::Eleven,
            Color::Twelve,
        ]
    }

    /// Convert to Index (bijection)
    #[inline]
    pub fn to_index(&self) -> Index {
        // Safe because Color and Index have the same valid range
        Index::from_value(self.value()).unwrap()
    }

    /// Create from Index (bijection)
    #[inline]
    pub fn from_index(index: Index) -> Self {
        // Safe because Color and Index have the same valid range
        Self::from_value(index.value()).unwrap()
    }

    // ========== Canonical name/hex (convenience methods using static data) ==========
    // For runtime-configurable lookups, use Registry instead

    /// Get the canonical color name (convenience method)
    /// For runtime-configurable lookups, use Registry::color_name()
    pub fn canonical_name(&self) -> &'static str {
        match self {
            Color::One => "Red",
            Color::Two => "Blue",
            Color::Three => "Yellow",
            Color::Four => "Green",
            Color::Five => "Purple",
            Color::Six => "Orange",
            Color::Seven => "Light Blue",
            Color::Eight => "Brown",
            Color::Nine => "Pink",
            Color::Ten => "White",
            Color::Eleven => "Grey/Silver",
            Color::Twelve => "Bright Orange",
        }
    }

    /// Get the canonical hex code (convenience method)
    /// For runtime-configurable lookups, use Registry::color_hex()
    pub fn canonical_hex(&self) -> &'static str {
        match self {
            Color::One => "#FF0000",
            Color::Two => "#0000FF",
            Color::Three => "#E6E600",
            Color::Four => "#00CC00",
            Color::Five => "#9370DB",
            Color::Six => "#FFA500",
            Color::Seven => "#00BFFF",
            Color::Eight => "#A0522D",
            Color::Nine => "#FF1493",
            Color::Ten => "#FFFFFF",
            Color::Eleven => "#C0C0C0",
            Color::Twelve => "#FFD700",
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl TryFrom<u8> for Color {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_value(value).ok_or("Color must be between 1 and 12")
    }
}

impl From<Color> for u8 {
    fn from(color: Color) -> u8 {
        color.value()
    }
}

impl From<Color> for Index {
    fn from(color: Color) -> Index {
        color.to_index()
    }
}

impl From<Index> for Color {
    fn from(index: Index) -> Color {
        Color::from_index(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_values() {
        assert_eq!(Color::One.value(), 1);
        assert_eq!(Color::Six.value(), 6);
        assert_eq!(Color::Twelve.value(), 12);
    }

    #[test]
    fn test_from_value() {
        assert_eq!(Color::from_value(1), Some(Color::One));
        assert_eq!(Color::from_value(12), Some(Color::Twelve));
        assert_eq!(Color::from_value(0), None);
        assert_eq!(Color::from_value(13), None);
    }

    #[test]
    fn test_zero_based_conversion() {
        assert_eq!(Color::One.to_zero_based(), 0);
        assert_eq!(Color::Twelve.to_zero_based(), 11);
        assert_eq!(Color::from_zero_based(0), Some(Color::One));
        assert_eq!(Color::from_zero_based(11), Some(Color::Twelve));
        assert_eq!(Color::from_zero_based(12), None);
    }

    #[test]
    fn test_all_colors() {
        let all = Color::all();
        assert_eq!(all.len(), 12);
        assert_eq!(all[0], Color::One);
        assert_eq!(all[11], Color::Twelve);
    }

    #[test]
    fn test_index_bijection() {
        for color in Color::all() {
            let index = color.to_index();
            let recovered = Color::from_index(index);
            assert_eq!(color, recovered);
        }
    }

    #[test]
    fn test_from_into_index() {
        let color = Color::Three;
        let index: Index = color.into();
        assert_eq!(index, Index::Three);

        let color2: Color = Index::Five.into();
        assert_eq!(color2, Color::Five);
    }

    #[test]
    fn test_canonical_names() {
        assert_eq!(Color::One.canonical_name(), "Red");
        assert_eq!(Color::Two.canonical_name(), "Blue");
        assert_eq!(Color::Three.canonical_name(), "Yellow");
    }

    #[test]
    fn test_canonical_hex() {
        assert_eq!(Color::One.canonical_hex(), "#FF0000");
        assert_eq!(Color::Two.canonical_hex(), "#0000FF");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Color::One), "1");
        assert_eq!(format!("{}", Color::Twelve), "12");
    }

    #[test]
    fn test_repr_u8() {
        assert_eq!(Color::One as u8, 1);
        assert_eq!(Color::Twelve as u8, 12);
    }

    #[test]
    fn test_ordering() {
        assert!(Color::One < Color::Two);
        assert!(Color::Eleven < Color::Twelve);
    }
}
