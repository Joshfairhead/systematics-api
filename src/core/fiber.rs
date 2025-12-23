use crate::core::essence::Essence;
use crate::core::system_content::SystemContent;
use crate::core::system_topology::Point3d;
use crate::core::Index;

/// Fiber is the fundamental unit containing index, coordinates, and content
#[derive(Debug, Clone)]
pub struct Fiber<T: SystemContent> {
    index: Index,
    coordinates: Point3d,
    content: T,
}

impl<T: SystemContent> Fiber<T> {
    /// Create a new Fiber
    pub fn new(index: Index, coordinates: Point3d, content: T) -> Self {
        Self {
            index,
            coordinates,
            content,
        }
    }

    /// Get the Index
    pub fn index(&self) -> Index {
        self.index
    }

    /// Get the coordinates
    pub fn coordinates(&self) -> Point3d {
        self.coordinates
    }

    /// Get the content
    pub fn content(&self) -> &T {
        &self.content
    }

    /// Get mutable content
    pub fn content_mut(&mut self) -> &mut T {
        &mut self.content
    }

    /// Take ownership of content
    pub fn into_content(self) -> T {
        self.content
    }

    // Getters that look up Essence data on the fly

    /// Get the system name from the Essence layer
    pub fn system_name(&self) -> &'static str {
        Essence::system_name(self.index)
    }

    /// Get the coherence attribute from the Essence layer
    pub fn coherence(&self) -> &'static str {
        Essence::coherence(self.index)
    }

    /// Get the term designation from the Essence layer (if available)
    pub fn term_designation(&self) -> Option<&'static str> {
        Essence::term_designation(self.index)
    }

    /// Get the connective designation from the Essence layer (if available)
    pub fn connective_designation(&self) -> Option<&'static str> {
        Essence::connective_designation(self.index)
    }

    /// Get full essence data
    pub fn essence(&self) -> &'static crate::core::essence::EssenceData {
        Essence::lookup(self.index)
    }
}

impl<T: SystemContent> PartialEq for Fiber<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
            && self.coordinates == other.coordinates
            && format!("{:?}", self.content) == format!("{:?}", other.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fiber_creation() {
        let index = Index::Three;
        let coords = Point3d::new(1.0, 0.0, 0.0);
        let content = String::from("Test");

        let fiber = Fiber::new(index, coords, content);

        assert_eq!(fiber.index().value(), 3);
        assert_eq!(fiber.coordinates().x, 1.0);
        assert_eq!(*fiber.content(), "Test");
    }

    #[test]
    fn test_essence_lookups() {
        let index = Index::One;
        let coords = Point3d::new(0.0, 0.0, 0.0);
        let fiber = Fiber::new(index, coords, String::from("Unity"));

        assert_eq!(fiber.system_name(), "Monad");
        assert_eq!(fiber.coherence(), "Unity");
        assert_eq!(fiber.term_designation(), Some("Totality"));
        assert_eq!(fiber.connective_designation(), Some("Unity"));
    }

    #[test]
    fn test_essence_without_designations() {
        let index = Index::Nine;
        let coords = Point3d::new(0.0, 0.0, 0.0);
        let fiber = Fiber::new(index, coords, String::from("Transform"));

        assert_eq!(fiber.system_name(), "Ennead");
        assert_eq!(fiber.coherence(), "Transformation");
        assert_eq!(fiber.term_designation(), None);
        assert_eq!(fiber.connective_designation(), None);
    }
}
