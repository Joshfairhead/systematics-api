use crate::core::{Index, Color, Node, Edge, Point, Line};
use crate::core::system_topology::Point3d;

/// Mapping represents a connection between two Fibers within a System.
///
/// In the simplicial category structure:
/// - Fibers are 0-simplices (vertices)
/// - Mappings are 1-simplices (edges connecting vertices)
///
/// A Mapping bundles together:
/// - Topological relationship: Edge (node to node)
/// - Geometric relationship: Line (point to point)
/// - Semantic relationship: Connective (term to term) - future extension
///
/// The mapping is parameterized by system order, determining which
/// sub-enum variants are valid for the connected nodes/points.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mapping {
    /// System order this mapping belongs to (1-12)
    order: u8,
    /// Source position (1-indexed within the system)
    from: Index,
    /// Target position (1-indexed within the system)
    to: Index,
}

impl Mapping {
    /// Create a new Mapping between two positions in a system
    ///
    /// Returns None if from == to (self-loops not allowed) or positions
    /// exceed the system order.
    pub fn new(order: u8, from: Index, to: Index) -> Option<Self> {
        // Validate positions are within order
        if from.value() > order || to.value() > order {
            return None;
        }
        // No self-loops
        if from == to {
            return None;
        }
        Some(Self { order, from, to })
    }

    /// Create a Mapping from an Edge
    pub fn from_edge(order: u8, edge: &Edge) -> Option<Self> {
        let from = edge.from().to_index();
        let to = edge.to().to_index();
        Self::new(order, from, to)
    }

    /// Create a Mapping from a Line
    pub fn from_line(order: u8, line: &Line) -> Option<Self> {
        let from = line.from().to_index();
        let to = line.to().to_index();
        Self::new(order, from, to)
    }

    // === Reference Accessors ===

    /// Get the system order
    pub fn order(&self) -> u8 {
        self.order
    }

    /// Get the source Index
    pub fn from_index(&self) -> Index {
        self.from
    }

    /// Get the target Index
    pub fn to_index(&self) -> Index {
        self.to
    }

    /// Get the source Color (derived from Index)
    pub fn from_color(&self) -> Color {
        Color::from_index(self.from)
    }

    /// Get the target Color (derived from Index)
    pub fn to_color(&self) -> Color {
        Color::from_index(self.to)
    }

    // === Topological Derivation ===

    /// Get the Edge (topological 1-simplex) for this mapping
    pub fn edge(&self) -> Option<Edge> {
        let from_node = Node::new(self.order, self.from.value())?;
        let to_node = Node::new(self.order, self.to.value())?;
        Edge::new(from_node, to_node)
    }

    /// Get the source Node
    pub fn from_node(&self) -> Option<Node> {
        Node::new(self.order, self.from.value())
    }

    /// Get the target Node
    pub fn to_node(&self) -> Option<Node> {
        Node::new(self.order, self.to.value())
    }

    // === Geometric Derivation ===

    /// Get the Line (geometric 1-simplex) for this mapping
    pub fn line(&self) -> Option<Line> {
        let from_point = Point::new(self.order, self.from.value())?;
        let to_point = Point::new(self.order, self.to.value())?;
        Line::new(from_point, to_point)
    }

    /// Get the source Point
    pub fn from_point(&self) -> Option<Point> {
        Point::new(self.order, self.from.value())
    }

    /// Get the target Point
    pub fn to_point(&self) -> Option<Point> {
        Point::new(self.order, self.to.value())
    }

    /// Get the geometric length of this mapping
    pub fn length(&self) -> Option<f64> {
        self.line().map(|l| l.length())
    }

    /// Get the source coordinates
    pub fn from_coordinates(&self) -> Option<Point3d> {
        self.from_point().map(|p| p.coordinates())
    }

    /// Get the target coordinates
    pub fn to_coordinates(&self) -> Option<Point3d> {
        self.to_point().map(|p| p.coordinates())
    }

    // === Utility Methods ===

    /// Check if this mapping is ordered (from < to)
    pub fn is_ordered(&self) -> bool {
        self.from.value() < self.to.value()
    }

    /// Get the canonical (ordered) form of this mapping
    pub fn canonical(&self) -> Self {
        if self.is_ordered() {
            *self
        } else {
            Self {
                order: self.order,
                from: self.to,
                to: self.from,
            }
        }
    }

    /// Get the reversed mapping
    pub fn reverse(&self) -> Self {
        Self {
            order: self.order,
            from: self.to,
            to: self.from,
        }
    }

    /// Generate all mappings for a given system order (complete graph)
    pub fn all_for_order(order: u8) -> Vec<Mapping> {
        let mut mappings = Vec::new();
        for i in 1..=order {
            for j in (i + 1)..=order {
                if let (Some(from), Some(to)) = (Index::from_value(i), Index::from_value(j)) {
                    if let Some(m) = Self::new(order, from, to) {
                        mappings.push(m);
                    }
                }
            }
        }
        mappings
    }

    /// Generate all mappings from a specific source position
    pub fn from_position(order: u8, from: Index) -> Vec<Mapping> {
        let mut mappings = Vec::new();
        for i in 1..=order {
            if let Some(to) = Index::from_value(i) {
                if from != to {
                    if let Some(m) = Self::new(order, from, to) {
                        mappings.push(m);
                    }
                }
            }
        }
        mappings
    }

    /// Generate all mappings to a specific target position
    pub fn to_position(order: u8, to: Index) -> Vec<Mapping> {
        let mut mappings = Vec::new();
        for i in 1..=order {
            if let Some(from) = Index::from_value(i) {
                if from != to {
                    if let Some(m) = Self::new(order, from, to) {
                        mappings.push(m);
                    }
                }
            }
        }
        mappings
    }
}

impl std::fmt::Display for Mapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Mapping({} → {}, order={})",
            self.from.value(),
            self.to.value(),
            self.order
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapping_creation() {
        let m = Mapping::new(3, Index::One, Index::Two);
        assert!(m.is_some());
        let m = m.unwrap();
        assert_eq!(m.order(), 3);
        assert_eq!(m.from_index().value(), 1);
        assert_eq!(m.to_index().value(), 2);
    }

    #[test]
    fn test_mapping_invalid_self_loop() {
        let m = Mapping::new(3, Index::One, Index::One);
        assert!(m.is_none());
    }

    #[test]
    fn test_mapping_invalid_position() {
        // Position 3 exceeds Dyad order of 2
        let m = Mapping::new(2, Index::One, Index::Three);
        assert!(m.is_none());
    }

    #[test]
    fn test_mapping_color_derivation() {
        let m = Mapping::new(3, Index::One, Index::Three).unwrap();
        assert_eq!(m.from_color().canonical_name(), "Red");
        assert_eq!(m.to_color().canonical_name(), "Yellow");
    }

    #[test]
    fn test_mapping_edge() {
        let m = Mapping::new(3, Index::One, Index::Two).unwrap();
        let edge = m.edge();
        assert!(edge.is_some());
        let edge = edge.unwrap();
        assert_eq!(edge.from().position(), 1);
        assert_eq!(edge.to().position(), 2);
    }

    #[test]
    fn test_mapping_line() {
        let m = Mapping::new(3, Index::One, Index::Two).unwrap();
        let line = m.line();
        assert!(line.is_some());
        let line = line.unwrap();
        assert_eq!(line.from().position(), 1);
        assert_eq!(line.to().position(), 2);
    }

    #[test]
    fn test_mapping_length() {
        let m = Mapping::new(3, Index::One, Index::Two).unwrap();
        let length = m.length();
        assert!(length.is_some());
        assert!(length.unwrap() > 0.0);
    }

    #[test]
    fn test_mapping_canonical() {
        let m = Mapping::new(3, Index::Three, Index::One).unwrap();
        assert!(!m.is_ordered());
        let canonical = m.canonical();
        assert!(canonical.is_ordered());
        assert_eq!(canonical.from_index().value(), 1);
        assert_eq!(canonical.to_index().value(), 3);
    }

    #[test]
    fn test_mapping_reverse() {
        let m = Mapping::new(3, Index::One, Index::Two).unwrap();
        let reversed = m.reverse();
        assert_eq!(reversed.from_index().value(), 2);
        assert_eq!(reversed.to_index().value(), 1);
    }

    #[test]
    fn test_all_mappings_for_order() {
        // Triad should have 3 mappings (complete graph K3 has 3 edges)
        let mappings = Mapping::all_for_order(3);
        assert_eq!(mappings.len(), 3);

        // Tetrad should have 6 mappings (K4 has 6 edges)
        let mappings = Mapping::all_for_order(4);
        assert_eq!(mappings.len(), 6);

        // Pentad should have 10 mappings (K5 has 10 edges)
        let mappings = Mapping::all_for_order(5);
        assert_eq!(mappings.len(), 10);
    }

    #[test]
    fn test_from_position() {
        // From position 1 in a Triad, should connect to 2 and 3
        let mappings = Mapping::from_position(3, Index::One);
        assert_eq!(mappings.len(), 2);
    }

    #[test]
    fn test_to_position() {
        // To position 3 in a Triad, should connect from 1 and 2
        let mappings = Mapping::to_position(3, Index::Three);
        assert_eq!(mappings.len(), 2);
    }

    #[test]
    fn test_from_edge() {
        let edge = Edge::new(
            Node::new(3, 1).unwrap(),
            Node::new(3, 2).unwrap(),
        ).unwrap();
        let m = Mapping::from_edge(3, &edge);
        assert!(m.is_some());
        let m = m.unwrap();
        assert_eq!(m.from_index().value(), 1);
        assert_eq!(m.to_index().value(), 2);
    }

    #[test]
    fn test_from_line() {
        let line = Line::new(
            Point::new(3, 1).unwrap(),
            Point::new(3, 2).unwrap(),
        ).unwrap();
        let m = Mapping::from_line(3, &line);
        assert!(m.is_some());
        let m = m.unwrap();
        assert_eq!(m.from_index().value(), 1);
        assert_eq!(m.to_index().value(), 2);
    }

    #[test]
    fn test_display() {
        let m = Mapping::new(3, Index::One, Index::Two).unwrap();
        assert_eq!(format!("{}", m), "Mapping(1 → 2, order=3)");
    }
}
