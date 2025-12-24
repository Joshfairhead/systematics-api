use crate::core::essence::Essence;
use crate::core::system_content::SystemContent;
use crate::core::system_topology::Point3d;
use crate::core::{Index, Color, Node, Point as GeometricPoint};

/// Fiber is the fundamental unit bundling together:
/// - Reference types: Index and Color (isomorphic identifiers)
/// - Topological data: Node (0-simplex in the simplicial structure)
/// - Geometric data: Point (coordinates in geometric realization)
/// - Semantic data: Content (vocabulary-specific meaning)
///
/// A Fiber exists at a specific position within a System of a given order.
/// The system order determines which sub-enum variants are valid.
#[derive(Debug, Clone)]
pub struct Fiber<T: SystemContent> {
    /// Primary reference - position within the system (1-indexed)
    index: Index,
    /// System order this fiber belongs to (1-12)
    order: u8,
    /// Geometric coordinates (from SystemTopology)
    coordinates: Point3d,
    /// Semantic content (vocabulary-dependent)
    content: T,
}

impl<T: SystemContent> Fiber<T> {
    /// Create a new Fiber with explicit order
    pub fn with_order(order: u8, index: Index, coordinates: Point3d, content: T) -> Self {
        Self {
            index,
            order,
            coordinates,
            content,
        }
    }

    /// Create a Fiber inferring order from index position (legacy compatibility)
    /// Assumes the index represents the system order itself (e.g., Index::Three = Triad)
    pub fn new(index: Index, coordinates: Point3d, content: T) -> Self {
        Self::with_order(index.value(), index, coordinates, content)
    }

    // === Reference Type Accessors ===

    /// Get the Index (primary reference)
    pub fn index(&self) -> Index {
        self.index
    }

    /// Get the Color (derived from Index via bijection)
    pub fn color(&self) -> Color {
        Color::from_index(self.index)
    }

    /// Get the system order this fiber belongs to
    pub fn order(&self) -> u8 {
        self.order
    }

    // === Topological Accessors ===

    /// Get the Node (topological 0-simplex)
    /// Returns the node for this position within the system of the given order
    pub fn node(&self) -> Option<Node> {
        Node::new(self.order, self.index.value())
    }

    // === Geometric Accessors ===

    /// Get the Point3d coordinates (from SystemTopology)
    pub fn coordinates(&self) -> Point3d {
        self.coordinates
    }

    /// Get the geometric Point enum for this position
    pub fn geometric_point(&self) -> Option<GeometricPoint> {
        GeometricPoint::new(self.order, self.index.value())
    }

    // === Semantic Accessors ===

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
            && self.order == other.order
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
        assert_eq!(fiber.order(), 3); // Inferred from index
        assert_eq!(fiber.coordinates().x, 1.0);
        assert_eq!(*fiber.content(), "Test");
    }

    #[test]
    fn test_fiber_with_explicit_order() {
        // A fiber at position 2 within a Triad (order 3)
        let fiber = Fiber::with_order(
            3,                              // Triad
            Index::Two,                     // Position 2
            Point3d::new(0.0, -1.0, 0.0),
            String::from("Function"),
        );

        assert_eq!(fiber.order(), 3);
        assert_eq!(fiber.index().value(), 2);
    }

    #[test]
    fn test_fiber_color_derivation() {
        let fiber = Fiber::new(Index::Three, Point3d::new(0.0, 0.0, 0.0), String::from("Test"));

        let color = fiber.color();
        assert_eq!(color.value(), 3);
        assert_eq!(color.canonical_name(), "Yellow");
    }

    #[test]
    fn test_fiber_node_access() {
        // Fiber at position 1 in a Triad
        let fiber = Fiber::with_order(3, Index::One, Point3d::new(0.0, 1.0, 0.0), String::from("Will"));

        let node = fiber.node();
        assert!(node.is_some());
        let node = node.unwrap();
        assert_eq!(node.order(), 3);
        assert_eq!(node.position(), 1);
    }

    #[test]
    fn test_fiber_geometric_point_access() {
        let fiber = Fiber::with_order(3, Index::Two, Point3d::new(0.0, -1.0, 0.0), String::from("Function"));

        let point = fiber.geometric_point();
        assert!(point.is_some());
        let point = point.unwrap();
        assert_eq!(point.order(), 3);
        assert_eq!(point.position(), 2);
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
