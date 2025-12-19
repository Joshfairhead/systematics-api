use std::fmt;

/// UniversalIndex represents the one-based index (1-12) for all systems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UniversalIndex(u8);

impl UniversalIndex {
    /// Create a new UniversalIndex from a one-based value (1-12)
    pub fn new(value: u8) -> Result<Self, &'static str> {
        if (1..=12).contains(&value) {
            Ok(UniversalIndex(value))
        } else {
            Err("UniversalIndex must be between 1 and 12")
        }
    }

    /// Get the one-based value
    pub fn value(&self) -> u8 {
        self.0
    }

    /// Convert to zero-based index for internal array access
    pub fn to_zero_based(&self) -> usize {
        (self.0 - 1) as usize
    }

    /// Create from zero-based index
    pub fn from_zero_based(index: usize) -> Result<Self, &'static str> {
        if index < 12 {
            Ok(UniversalIndex((index + 1) as u8))
        } else {
            Err("Zero-based index must be less than 12")
        }
    }
}

impl fmt::Display for UniversalIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for UniversalIndex {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        UniversalIndex::new(value)
    }
}

impl From<UniversalIndex> for u8 {
    fn from(index: UniversalIndex) -> u8 {
        index.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_index() {
        assert!(UniversalIndex::new(1).is_ok());
        assert!(UniversalIndex::new(12).is_ok());
        assert!(UniversalIndex::new(6).is_ok());
    }

    #[test]
    fn test_invalid_index() {
        assert!(UniversalIndex::new(0).is_err());
        assert!(UniversalIndex::new(13).is_err());
    }

    #[test]
    fn test_conversions() {
        let index = UniversalIndex::new(1).unwrap();
        assert_eq!(index.value(), 1);
        assert_eq!(index.to_zero_based(), 0);

        let index = UniversalIndex::new(12).unwrap();
        assert_eq!(index.value(), 12);
        assert_eq!(index.to_zero_based(), 11);
    }
}
