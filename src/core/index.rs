//! Index: The fundamental numeric reference type for Systematics.
//!
//! Index represents positions 1-12 in the universal ordering.
//! It is isomorphic with Color and serves as the primary reference
//! for all content types (Node, Point, Term, etc.)

use std::fmt;

/// Index represents a one-based position (1-12) in the universal ordering.
///
/// This is the primary reference type - all other types (Color, Node, Point, Term)
/// can be derived from or mapped back to an Index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Index {
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

impl Index {
    /// Get the numeric value (1-12)
    #[inline]
    pub fn value(&self) -> u8 {
        *self as u8
    }

    /// Create an Index from a numeric value (1-12)
    /// Returns None if the value is out of range
    pub fn from_value(n: u8) -> Option<Self> {
        match n {
            1 => Some(Index::One),
            2 => Some(Index::Two),
            3 => Some(Index::Three),
            4 => Some(Index::Four),
            5 => Some(Index::Five),
            6 => Some(Index::Six),
            7 => Some(Index::Seven),
            8 => Some(Index::Eight),
            9 => Some(Index::Nine),
            10 => Some(Index::Ten),
            11 => Some(Index::Eleven),
            12 => Some(Index::Twelve),
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

    /// Map to standard simplex category Δ (0-indexed).
    ///
    /// In the simplex category, objects are denoted [0], [1], [2], etc.
    /// This maps our 1-based Index to the standard 0-based notation:
    /// - Index::One → Δ[0]
    /// - Index::Two → Δ[1]
    /// - Index::Three → Δ[2]
    #[inline]
    pub fn to_delta(&self) -> u8 {
        self.value() - 1
    }

    /// Create from standard simplex category Δ (0-indexed).
    ///
    /// Maps from the standard 0-based simplex category notation:
    /// - Δ[0] → Index::One
    /// - Δ[1] → Index::Two
    /// - Δ[2] → Index::Three
    pub fn from_delta(n: u8) -> Option<Self> {
        Self::from_value(n + 1)
    }

    /// Get all indices in order
    pub fn all() -> [Index; 12] {
        [
            Index::One,
            Index::Two,
            Index::Three,
            Index::Four,
            Index::Five,
            Index::Six,
            Index::Seven,
            Index::Eight,
            Index::Nine,
            Index::Ten,
            Index::Eleven,
            Index::Twelve,
        ]
    }

    /// Get the system name for this index (canonical)
    pub fn system_name(&self) -> &'static str {
        match self {
            Index::One => "Monad",
            Index::Two => "Dyad",
            Index::Three => "Triad",
            Index::Four => "Tetrad",
            Index::Five => "Pentad",
            Index::Six => "Hexad",
            Index::Seven => "Heptad",
            Index::Eight => "Octad",
            Index::Nine => "Ennead",
            Index::Ten => "Decad",
            Index::Eleven => "Undecad",
            Index::Twelve => "Dodecad",
        }
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl TryFrom<u8> for Index {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_value(value).ok_or("Index must be between 1 and 12")
    }
}

impl From<Index> for u8 {
    fn from(index: Index) -> u8 {
        index.value()
    }
}

impl From<Index> for usize {
    fn from(index: Index) -> usize {
        index.value() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_values() {
        assert_eq!(Index::One.value(), 1);
        assert_eq!(Index::Six.value(), 6);
        assert_eq!(Index::Twelve.value(), 12);
    }

    #[test]
    fn test_from_value() {
        assert_eq!(Index::from_value(1), Some(Index::One));
        assert_eq!(Index::from_value(12), Some(Index::Twelve));
        assert_eq!(Index::from_value(0), None);
        assert_eq!(Index::from_value(13), None);
    }

    #[test]
    fn test_zero_based_conversion() {
        assert_eq!(Index::One.to_zero_based(), 0);
        assert_eq!(Index::Twelve.to_zero_based(), 11);
        assert_eq!(Index::from_zero_based(0), Some(Index::One));
        assert_eq!(Index::from_zero_based(11), Some(Index::Twelve));
        assert_eq!(Index::from_zero_based(12), None);
    }

    #[test]
    fn test_all_indices() {
        let all = Index::all();
        assert_eq!(all.len(), 12);
        assert_eq!(all[0], Index::One);
        assert_eq!(all[11], Index::Twelve);
    }

    #[test]
    fn test_system_names() {
        assert_eq!(Index::One.system_name(), "Monad");
        assert_eq!(Index::Three.system_name(), "Triad");
        assert_eq!(Index::Twelve.system_name(), "Dodecad");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Index::One), "1");
        assert_eq!(format!("{}", Index::Twelve), "12");
    }

    #[test]
    fn test_try_from() {
        assert_eq!(Index::try_from(1u8), Ok(Index::One));
        assert_eq!(Index::try_from(12u8), Ok(Index::Twelve));
        assert!(Index::try_from(0u8).is_err());
        assert!(Index::try_from(13u8).is_err());
    }

    #[test]
    fn test_ordering() {
        assert!(Index::One < Index::Two);
        assert!(Index::Eleven < Index::Twelve);
    }

    #[test]
    fn test_repr_u8() {
        // Verify that the enum discriminants are correctly set
        assert_eq!(Index::One as u8, 1);
        assert_eq!(Index::Twelve as u8, 12);
    }

    #[test]
    fn test_delta_conversion() {
        // to_delta maps 1-based Index to 0-based simplex category
        assert_eq!(Index::One.to_delta(), 0);
        assert_eq!(Index::Two.to_delta(), 1);
        assert_eq!(Index::Three.to_delta(), 2);
        assert_eq!(Index::Twelve.to_delta(), 11);

        // from_delta maps 0-based simplex category to 1-based Index
        assert_eq!(Index::from_delta(0), Some(Index::One));
        assert_eq!(Index::from_delta(1), Some(Index::Two));
        assert_eq!(Index::from_delta(2), Some(Index::Three));
        assert_eq!(Index::from_delta(11), Some(Index::Twelve));
        assert_eq!(Index::from_delta(12), None); // Out of range
    }

    #[test]
    fn test_delta_roundtrip() {
        // Verify roundtrip for all indices
        for idx in Index::all() {
            let delta = idx.to_delta();
            let recovered = Index::from_delta(delta);
            assert_eq!(recovered, Some(idx));
        }
    }
}
