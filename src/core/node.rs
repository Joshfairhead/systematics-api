//! Node: Topological vertex type for Systematics.
//!
//! Nodes represent vertices in the simplicial structure of each system.
//! The Index forms a simplicial category where:
//! - Nodes are 0-simplices (vertices)
//! - Edges are 1-simplices (connections between vertices)
//! - Higher faces exist for systems of order > 2
//!
//! Each system order has its own node sub-enum, all unified under the
//! top-level Node enum with discriminants matching the system order.

use crate::core::Index;
use std::fmt;

// ============================================================================
// Sub-enums for each system order (0-simplices within each system)
// ============================================================================

/// Monadic node (single vertex - the 0-simplex)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MonadicNode {
    One = 1,
}

/// Dyadic nodes (two vertices - endpoints of a 1-simplex)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DyadicNode {
    One = 1,
    Two = 2,
}

/// Triadic nodes (three vertices - 2-simplex/triangle)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TriadicNode {
    One = 1,
    Two = 2,
    Three = 3,
}

/// Tetradic nodes (four vertices - 3-simplex/tetrahedron)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TetradicNode {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

/// Pentadic nodes (five vertices - 4-simplex)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PentadicNode {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

/// Hexadic nodes (six vertices - 5-simplex)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HexadicNode {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
}

/// Heptadic nodes (seven vertices - 6-simplex)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HeptadicNode {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
}

/// Octadic nodes (eight vertices - 7-simplex)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OctadicNode {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
}

/// Enneadic nodes (nine vertices - 8-simplex)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EnneadicNode {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

/// Decadic nodes (ten vertices - 9-simplex)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DecadicNode {
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
}

/// Undecadic nodes (eleven vertices - 10-simplex)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum UndecadicNode {
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
}

/// Dodecadic nodes (twelve vertices - 11-simplex)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DodecadicNode {
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

// ============================================================================
// Top-level Node enum (discriminant = system order)
// ============================================================================

/// Node represents a vertex in the simplicial structure.
/// The outer enum discriminant indicates the system order (1-12).
/// The inner enum indicates the position within that system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Node {
    Monadic(MonadicNode),
    Dyadic(DyadicNode),
    Triadic(TriadicNode),
    Tetradic(TetradicNode),
    Pentadic(PentadicNode),
    Hexadic(HexadicNode),
    Heptadic(HeptadicNode),
    Octadic(OctadicNode),
    Enneadic(EnneadicNode),
    Decadic(DecadicNode),
    Undecadic(UndecadicNode),
    Dodecadic(DodecadicNode),
}

impl Node {
    /// Get the system order this node belongs to
    pub fn order(&self) -> u8 {
        match self {
            Node::Monadic(_) => 1,
            Node::Dyadic(_) => 2,
            Node::Triadic(_) => 3,
            Node::Tetradic(_) => 4,
            Node::Pentadic(_) => 5,
            Node::Hexadic(_) => 6,
            Node::Heptadic(_) => 7,
            Node::Octadic(_) => 8,
            Node::Enneadic(_) => 9,
            Node::Decadic(_) => 10,
            Node::Undecadic(_) => 11,
            Node::Dodecadic(_) => 12,
        }
    }

    /// Get the position within the system (1-based)
    pub fn position(&self) -> u8 {
        match self {
            Node::Monadic(n) => *n as u8,
            Node::Dyadic(n) => *n as u8,
            Node::Triadic(n) => *n as u8,
            Node::Tetradic(n) => *n as u8,
            Node::Pentadic(n) => *n as u8,
            Node::Hexadic(n) => *n as u8,
            Node::Heptadic(n) => *n as u8,
            Node::Octadic(n) => *n as u8,
            Node::Enneadic(n) => *n as u8,
            Node::Decadic(n) => *n as u8,
            Node::Undecadic(n) => *n as u8,
            Node::Dodecadic(n) => *n as u8,
        }
    }

    /// Convert to Index (the universal reference)
    pub fn to_index(&self) -> Index {
        Index::from_value(self.position()).unwrap()
    }

    /// Create a Node from system order and position
    pub fn new(order: u8, position: u8) -> Option<Self> {
        if position < 1 || position > order || order > 12 {
            return None;
        }

        match order {
            1 => Some(Node::Monadic(MonadicNode::One)),
            2 => match position {
                1 => Some(Node::Dyadic(DyadicNode::One)),
                2 => Some(Node::Dyadic(DyadicNode::Two)),
                _ => None,
            },
            3 => match position {
                1 => Some(Node::Triadic(TriadicNode::One)),
                2 => Some(Node::Triadic(TriadicNode::Two)),
                3 => Some(Node::Triadic(TriadicNode::Three)),
                _ => None,
            },
            4 => match position {
                1 => Some(Node::Tetradic(TetradicNode::One)),
                2 => Some(Node::Tetradic(TetradicNode::Two)),
                3 => Some(Node::Tetradic(TetradicNode::Three)),
                4 => Some(Node::Tetradic(TetradicNode::Four)),
                _ => None,
            },
            5 => match position {
                1 => Some(Node::Pentadic(PentadicNode::One)),
                2 => Some(Node::Pentadic(PentadicNode::Two)),
                3 => Some(Node::Pentadic(PentadicNode::Three)),
                4 => Some(Node::Pentadic(PentadicNode::Four)),
                5 => Some(Node::Pentadic(PentadicNode::Five)),
                _ => None,
            },
            6 => match position {
                1 => Some(Node::Hexadic(HexadicNode::One)),
                2 => Some(Node::Hexadic(HexadicNode::Two)),
                3 => Some(Node::Hexadic(HexadicNode::Three)),
                4 => Some(Node::Hexadic(HexadicNode::Four)),
                5 => Some(Node::Hexadic(HexadicNode::Five)),
                6 => Some(Node::Hexadic(HexadicNode::Six)),
                _ => None,
            },
            7 => match position {
                1 => Some(Node::Heptadic(HeptadicNode::One)),
                2 => Some(Node::Heptadic(HeptadicNode::Two)),
                3 => Some(Node::Heptadic(HeptadicNode::Three)),
                4 => Some(Node::Heptadic(HeptadicNode::Four)),
                5 => Some(Node::Heptadic(HeptadicNode::Five)),
                6 => Some(Node::Heptadic(HeptadicNode::Six)),
                7 => Some(Node::Heptadic(HeptadicNode::Seven)),
                _ => None,
            },
            8 => match position {
                1 => Some(Node::Octadic(OctadicNode::One)),
                2 => Some(Node::Octadic(OctadicNode::Two)),
                3 => Some(Node::Octadic(OctadicNode::Three)),
                4 => Some(Node::Octadic(OctadicNode::Four)),
                5 => Some(Node::Octadic(OctadicNode::Five)),
                6 => Some(Node::Octadic(OctadicNode::Six)),
                7 => Some(Node::Octadic(OctadicNode::Seven)),
                8 => Some(Node::Octadic(OctadicNode::Eight)),
                _ => None,
            },
            9 => match position {
                1 => Some(Node::Enneadic(EnneadicNode::One)),
                2 => Some(Node::Enneadic(EnneadicNode::Two)),
                3 => Some(Node::Enneadic(EnneadicNode::Three)),
                4 => Some(Node::Enneadic(EnneadicNode::Four)),
                5 => Some(Node::Enneadic(EnneadicNode::Five)),
                6 => Some(Node::Enneadic(EnneadicNode::Six)),
                7 => Some(Node::Enneadic(EnneadicNode::Seven)),
                8 => Some(Node::Enneadic(EnneadicNode::Eight)),
                9 => Some(Node::Enneadic(EnneadicNode::Nine)),
                _ => None,
            },
            10 => match position {
                1 => Some(Node::Decadic(DecadicNode::One)),
                2 => Some(Node::Decadic(DecadicNode::Two)),
                3 => Some(Node::Decadic(DecadicNode::Three)),
                4 => Some(Node::Decadic(DecadicNode::Four)),
                5 => Some(Node::Decadic(DecadicNode::Five)),
                6 => Some(Node::Decadic(DecadicNode::Six)),
                7 => Some(Node::Decadic(DecadicNode::Seven)),
                8 => Some(Node::Decadic(DecadicNode::Eight)),
                9 => Some(Node::Decadic(DecadicNode::Nine)),
                10 => Some(Node::Decadic(DecadicNode::Ten)),
                _ => None,
            },
            11 => match position {
                1 => Some(Node::Undecadic(UndecadicNode::One)),
                2 => Some(Node::Undecadic(UndecadicNode::Two)),
                3 => Some(Node::Undecadic(UndecadicNode::Three)),
                4 => Some(Node::Undecadic(UndecadicNode::Four)),
                5 => Some(Node::Undecadic(UndecadicNode::Five)),
                6 => Some(Node::Undecadic(UndecadicNode::Six)),
                7 => Some(Node::Undecadic(UndecadicNode::Seven)),
                8 => Some(Node::Undecadic(UndecadicNode::Eight)),
                9 => Some(Node::Undecadic(UndecadicNode::Nine)),
                10 => Some(Node::Undecadic(UndecadicNode::Ten)),
                11 => Some(Node::Undecadic(UndecadicNode::Eleven)),
                _ => None,
            },
            12 => match position {
                1 => Some(Node::Dodecadic(DodecadicNode::One)),
                2 => Some(Node::Dodecadic(DodecadicNode::Two)),
                3 => Some(Node::Dodecadic(DodecadicNode::Three)),
                4 => Some(Node::Dodecadic(DodecadicNode::Four)),
                5 => Some(Node::Dodecadic(DodecadicNode::Five)),
                6 => Some(Node::Dodecadic(DodecadicNode::Six)),
                7 => Some(Node::Dodecadic(DodecadicNode::Seven)),
                8 => Some(Node::Dodecadic(DodecadicNode::Eight)),
                9 => Some(Node::Dodecadic(DodecadicNode::Nine)),
                10 => Some(Node::Dodecadic(DodecadicNode::Ten)),
                11 => Some(Node::Dodecadic(DodecadicNode::Eleven)),
                12 => Some(Node::Dodecadic(DodecadicNode::Twelve)),
                _ => None,
            },
            _ => None,
        }
    }

    /// Create a Node from an Index within a given system order
    pub fn from_index(index: Index, order: u8) -> Option<Self> {
        Self::new(order, index.value())
    }

    /// Get all nodes for a given system order
    pub fn all_for_order(order: u8) -> Vec<Node> {
        (1..=order)
            .filter_map(|pos| Node::new(order, pos))
            .collect()
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node({}, {})", self.order(), self.position())
    }
}

// ============================================================================
// Edge type (1-simplex connecting two nodes)
// ============================================================================

/// Edge represents a 1-simplex connecting two nodes within the same system.
/// In the simplicial structure, edges form the 1-skeleton of the complex.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge {
    from: Node,
    to: Node,
}

impl Edge {
    /// Create a new edge between two nodes
    /// Returns None if nodes are from different systems or are the same node
    pub fn new(from: Node, to: Node) -> Option<Self> {
        if from.order() != to.order() {
            return None; // Nodes must be in same system
        }
        if from.position() == to.position() {
            return None; // No self-loops
        }
        Some(Edge { from, to })
    }

    /// Get the source node
    pub fn from(&self) -> Node {
        self.from
    }

    /// Get the target node
    pub fn to(&self) -> Node {
        self.to
    }

    /// Get the system order
    pub fn order(&self) -> u8 {
        self.from.order()
    }

    /// Get the source index
    pub fn from_index(&self) -> Index {
        self.from.to_index()
    }

    /// Get the target index
    pub fn to_index(&self) -> Index {
        self.to.to_index()
    }

    /// Create from indices within a system order
    pub fn from_indices(from: Index, to: Index, order: u8) -> Option<Self> {
        let from_node = Node::from_index(from, order)?;
        let to_node = Node::from_index(to, order)?;
        Self::new(from_node, to_node)
    }

    /// Get all edges for a complete graph of given order
    pub fn all_for_order(order: u8) -> Vec<Edge> {
        let nodes = Node::all_for_order(order);
        let mut edges = Vec::new();
        for (i, from) in nodes.iter().enumerate() {
            for to in nodes.iter().skip(i + 1) {
                if let Some(edge) = Edge::new(*from, *to) {
                    edges.push(edge);
                }
            }
        }
        edges
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Edge({} -> {})", self.from.position(), self.to.position())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::new(3, 2).unwrap();
        assert_eq!(node.order(), 3);
        assert_eq!(node.position(), 2);
        assert!(matches!(node, Node::Triadic(TriadicNode::Two)));
    }

    #[test]
    fn test_node_invalid() {
        assert!(Node::new(3, 4).is_none()); // Position exceeds order
        assert!(Node::new(0, 1).is_none()); // Invalid order
        assert!(Node::new(13, 1).is_none()); // Order too high
        assert!(Node::new(3, 0).is_none()); // Position 0 invalid
    }

    #[test]
    fn test_node_to_index() {
        let node = Node::new(5, 3).unwrap();
        let index = node.to_index();
        assert_eq!(index, Index::Three);
    }

    #[test]
    fn test_node_from_index() {
        let node = Node::from_index(Index::Two, 4).unwrap();
        assert_eq!(node.order(), 4);
        assert_eq!(node.position(), 2);
    }

    #[test]
    fn test_all_nodes_for_order() {
        let nodes = Node::all_for_order(3);
        assert_eq!(nodes.len(), 3);
        assert_eq!(nodes[0].position(), 1);
        assert_eq!(nodes[1].position(), 2);
        assert_eq!(nodes[2].position(), 3);
    }

    #[test]
    fn test_edge_creation() {
        let from = Node::new(3, 1).unwrap();
        let to = Node::new(3, 2).unwrap();
        let edge = Edge::new(from, to).unwrap();
        assert_eq!(edge.order(), 3);
        assert_eq!(edge.from_index(), Index::One);
        assert_eq!(edge.to_index(), Index::Two);
    }

    #[test]
    fn test_edge_invalid() {
        let node1 = Node::new(3, 1).unwrap();
        let node2 = Node::new(4, 1).unwrap(); // Different order
        assert!(Edge::new(node1, node2).is_none());

        let node3 = Node::new(3, 1).unwrap();
        assert!(Edge::new(node1, node3).is_none()); // Same position (self-loop)
    }

    #[test]
    fn test_all_edges_for_order() {
        // Triad: 3 nodes, 3 edges (complete graph K3)
        let edges = Edge::all_for_order(3);
        assert_eq!(edges.len(), 3);

        // Tetrad: 4 nodes, 6 edges (complete graph K4)
        let edges = Edge::all_for_order(4);
        assert_eq!(edges.len(), 6);

        // General formula: n(n-1)/2 edges for Kn
        let edges = Edge::all_for_order(6);
        assert_eq!(edges.len(), 15); // 6*5/2 = 15
    }

    #[test]
    fn test_sub_enum_discriminants() {
        assert_eq!(TriadicNode::One as u8, 1);
        assert_eq!(TriadicNode::Two as u8, 2);
        assert_eq!(TriadicNode::Three as u8, 3);

        assert_eq!(DodecadicNode::Twelve as u8, 12);
    }
}
