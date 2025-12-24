//! Point: Geometric vertex type for Systematics.
//!
//! Points represent geometric positions in the coordinate space.
//! They are the geometric realization of Nodes - while Nodes are purely
//! topological (combinatorial), Points have actual coordinates.
//!
//! The simplicial category structure:
//! - Points are 0-simplices with coordinates
//! - Lines are 1-simplices connecting two points
//! - Higher faces have geometric extent
//!
//! Coordinates are looked up from the Registry/SystemTopology.

use crate::core::Index;
use crate::core::system_topology::{Point3d, SystemTopology};
use std::fmt;

// ============================================================================
// Sub-enums for each system order (geometric 0-simplices)
// ============================================================================

/// Monadic point (single vertex at origin)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MonadicPoint {
    One = 1,
}

/// Dyadic points (two vertices)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DyadicPoint {
    One = 1,
    Two = 2,
}

/// Triadic points (three vertices - triangle)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TriadicPoint {
    One = 1,
    Two = 2,
    Three = 3,
}

/// Tetradic points (four vertices - tetrahedron)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TetradicPoint {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

/// Pentadic points (five vertices - pentagon/pentahedron)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PentadicPoint {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

/// Hexadic points (six vertices - hexagon/hexahedron)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HexadicPoint {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
}

/// Heptadic points (seven vertices)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HeptadicPoint {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
}

/// Octadic points (eight vertices)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OctadicPoint {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
}

/// Enneadic points (nine vertices)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EnneadicPoint {
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

/// Decadic points (ten vertices)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DecadicPoint {
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

/// Undecadic points (eleven vertices)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum UndecadicPoint {
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

/// Dodecadic points (twelve vertices)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DodecadicPoint {
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
// Top-level Point enum
// ============================================================================

/// Point represents a geometric vertex with coordinates.
/// The outer enum indicates the system order (1-12).
/// The inner enum indicates the position within that system.
/// Actual coordinates are looked up from SystemTopology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Point {
    Monadic(MonadicPoint),
    Dyadic(DyadicPoint),
    Triadic(TriadicPoint),
    Tetradic(TetradicPoint),
    Pentadic(PentadicPoint),
    Hexadic(HexadicPoint),
    Heptadic(HeptadicPoint),
    Octadic(OctadicPoint),
    Enneadic(EnneadicPoint),
    Decadic(DecadicPoint),
    Undecadic(UndecadicPoint),
    Dodecadic(DodecadicPoint),
}

impl Point {
    /// Get the system order this point belongs to
    pub fn order(&self) -> u8 {
        match self {
            Point::Monadic(_) => 1,
            Point::Dyadic(_) => 2,
            Point::Triadic(_) => 3,
            Point::Tetradic(_) => 4,
            Point::Pentadic(_) => 5,
            Point::Hexadic(_) => 6,
            Point::Heptadic(_) => 7,
            Point::Octadic(_) => 8,
            Point::Enneadic(_) => 9,
            Point::Decadic(_) => 10,
            Point::Undecadic(_) => 11,
            Point::Dodecadic(_) => 12,
        }
    }

    /// Get the position within the system (1-based)
    pub fn position(&self) -> u8 {
        match self {
            Point::Monadic(p) => *p as u8,
            Point::Dyadic(p) => *p as u8,
            Point::Triadic(p) => *p as u8,
            Point::Tetradic(p) => *p as u8,
            Point::Pentadic(p) => *p as u8,
            Point::Hexadic(p) => *p as u8,
            Point::Heptadic(p) => *p as u8,
            Point::Octadic(p) => *p as u8,
            Point::Enneadic(p) => *p as u8,
            Point::Decadic(p) => *p as u8,
            Point::Undecadic(p) => *p as u8,
            Point::Dodecadic(p) => *p as u8,
        }
    }

    /// Convert to Index (the universal reference)
    pub fn to_index(&self) -> Index {
        Index::from_value(self.position()).unwrap()
    }

    /// Get the actual 3D coordinates for this point
    pub fn coordinates(&self) -> Point3d {
        let index = self.to_index();
        SystemTopology::get_coordinate(index, self.order())
            .expect("Valid point should have valid coordinates")
    }

    /// Create a Point from system order and position
    pub fn new(order: u8, position: u8) -> Option<Self> {
        if position < 1 || position > order || order > 12 {
            return None;
        }

        match order {
            1 => Some(Point::Monadic(MonadicPoint::One)),
            2 => match position {
                1 => Some(Point::Dyadic(DyadicPoint::One)),
                2 => Some(Point::Dyadic(DyadicPoint::Two)),
                _ => None,
            },
            3 => match position {
                1 => Some(Point::Triadic(TriadicPoint::One)),
                2 => Some(Point::Triadic(TriadicPoint::Two)),
                3 => Some(Point::Triadic(TriadicPoint::Three)),
                _ => None,
            },
            4 => match position {
                1 => Some(Point::Tetradic(TetradicPoint::One)),
                2 => Some(Point::Tetradic(TetradicPoint::Two)),
                3 => Some(Point::Tetradic(TetradicPoint::Three)),
                4 => Some(Point::Tetradic(TetradicPoint::Four)),
                _ => None,
            },
            5 => match position {
                1 => Some(Point::Pentadic(PentadicPoint::One)),
                2 => Some(Point::Pentadic(PentadicPoint::Two)),
                3 => Some(Point::Pentadic(PentadicPoint::Three)),
                4 => Some(Point::Pentadic(PentadicPoint::Four)),
                5 => Some(Point::Pentadic(PentadicPoint::Five)),
                _ => None,
            },
            6 => match position {
                1 => Some(Point::Hexadic(HexadicPoint::One)),
                2 => Some(Point::Hexadic(HexadicPoint::Two)),
                3 => Some(Point::Hexadic(HexadicPoint::Three)),
                4 => Some(Point::Hexadic(HexadicPoint::Four)),
                5 => Some(Point::Hexadic(HexadicPoint::Five)),
                6 => Some(Point::Hexadic(HexadicPoint::Six)),
                _ => None,
            },
            7 => match position {
                1 => Some(Point::Heptadic(HeptadicPoint::One)),
                2 => Some(Point::Heptadic(HeptadicPoint::Two)),
                3 => Some(Point::Heptadic(HeptadicPoint::Three)),
                4 => Some(Point::Heptadic(HeptadicPoint::Four)),
                5 => Some(Point::Heptadic(HeptadicPoint::Five)),
                6 => Some(Point::Heptadic(HeptadicPoint::Six)),
                7 => Some(Point::Heptadic(HeptadicPoint::Seven)),
                _ => None,
            },
            8 => match position {
                1 => Some(Point::Octadic(OctadicPoint::One)),
                2 => Some(Point::Octadic(OctadicPoint::Two)),
                3 => Some(Point::Octadic(OctadicPoint::Three)),
                4 => Some(Point::Octadic(OctadicPoint::Four)),
                5 => Some(Point::Octadic(OctadicPoint::Five)),
                6 => Some(Point::Octadic(OctadicPoint::Six)),
                7 => Some(Point::Octadic(OctadicPoint::Seven)),
                8 => Some(Point::Octadic(OctadicPoint::Eight)),
                _ => None,
            },
            9 => match position {
                1 => Some(Point::Enneadic(EnneadicPoint::One)),
                2 => Some(Point::Enneadic(EnneadicPoint::Two)),
                3 => Some(Point::Enneadic(EnneadicPoint::Three)),
                4 => Some(Point::Enneadic(EnneadicPoint::Four)),
                5 => Some(Point::Enneadic(EnneadicPoint::Five)),
                6 => Some(Point::Enneadic(EnneadicPoint::Six)),
                7 => Some(Point::Enneadic(EnneadicPoint::Seven)),
                8 => Some(Point::Enneadic(EnneadicPoint::Eight)),
                9 => Some(Point::Enneadic(EnneadicPoint::Nine)),
                _ => None,
            },
            10 => match position {
                1 => Some(Point::Decadic(DecadicPoint::One)),
                2 => Some(Point::Decadic(DecadicPoint::Two)),
                3 => Some(Point::Decadic(DecadicPoint::Three)),
                4 => Some(Point::Decadic(DecadicPoint::Four)),
                5 => Some(Point::Decadic(DecadicPoint::Five)),
                6 => Some(Point::Decadic(DecadicPoint::Six)),
                7 => Some(Point::Decadic(DecadicPoint::Seven)),
                8 => Some(Point::Decadic(DecadicPoint::Eight)),
                9 => Some(Point::Decadic(DecadicPoint::Nine)),
                10 => Some(Point::Decadic(DecadicPoint::Ten)),
                _ => None,
            },
            11 => match position {
                1 => Some(Point::Undecadic(UndecadicPoint::One)),
                2 => Some(Point::Undecadic(UndecadicPoint::Two)),
                3 => Some(Point::Undecadic(UndecadicPoint::Three)),
                4 => Some(Point::Undecadic(UndecadicPoint::Four)),
                5 => Some(Point::Undecadic(UndecadicPoint::Five)),
                6 => Some(Point::Undecadic(UndecadicPoint::Six)),
                7 => Some(Point::Undecadic(UndecadicPoint::Seven)),
                8 => Some(Point::Undecadic(UndecadicPoint::Eight)),
                9 => Some(Point::Undecadic(UndecadicPoint::Nine)),
                10 => Some(Point::Undecadic(UndecadicPoint::Ten)),
                11 => Some(Point::Undecadic(UndecadicPoint::Eleven)),
                _ => None,
            },
            12 => match position {
                1 => Some(Point::Dodecadic(DodecadicPoint::One)),
                2 => Some(Point::Dodecadic(DodecadicPoint::Two)),
                3 => Some(Point::Dodecadic(DodecadicPoint::Three)),
                4 => Some(Point::Dodecadic(DodecadicPoint::Four)),
                5 => Some(Point::Dodecadic(DodecadicPoint::Five)),
                6 => Some(Point::Dodecadic(DodecadicPoint::Six)),
                7 => Some(Point::Dodecadic(DodecadicPoint::Seven)),
                8 => Some(Point::Dodecadic(DodecadicPoint::Eight)),
                9 => Some(Point::Dodecadic(DodecadicPoint::Nine)),
                10 => Some(Point::Dodecadic(DodecadicPoint::Ten)),
                11 => Some(Point::Dodecadic(DodecadicPoint::Eleven)),
                12 => Some(Point::Dodecadic(DodecadicPoint::Twelve)),
                _ => None,
            },
            _ => None,
        }
    }

    /// Create a Point from an Index within a given system order
    pub fn from_index(index: Index, order: u8) -> Option<Self> {
        Self::new(order, index.value())
    }

    /// Get all points for a given system order
    pub fn all_for_order(order: u8) -> Vec<Point> {
        (1..=order)
            .filter_map(|pos| Point::new(order, pos))
            .collect()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let coords = self.coordinates();
        write!(
            f,
            "Point({}, {}) @ ({:.3}, {:.3}, {:.3})",
            self.order(),
            self.position(),
            coords.x,
            coords.y,
            coords.z
        )
    }
}

// ============================================================================
// Line type (geometric 1-simplex connecting two points)
// ============================================================================

/// Line represents a geometric 1-simplex connecting two points.
/// Unlike Edge (topological), Line has actual geometric extent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Line {
    from: Point,
    to: Point,
}

impl Line {
    /// Create a new line between two points
    /// Returns None if points are from different systems or are the same point
    pub fn new(from: Point, to: Point) -> Option<Self> {
        if from.order() != to.order() {
            return None; // Points must be in same system
        }
        if from.position() == to.position() {
            return None; // No zero-length lines
        }
        Some(Line { from, to })
    }

    /// Get the start point
    pub fn from(&self) -> Point {
        self.from
    }

    /// Get the end point
    pub fn to(&self) -> Point {
        self.to
    }

    /// Get the system order
    pub fn order(&self) -> u8 {
        self.from.order()
    }

    /// Get start coordinates
    pub fn from_coordinates(&self) -> Point3d {
        self.from.coordinates()
    }

    /// Get end coordinates
    pub fn to_coordinates(&self) -> Point3d {
        self.to.coordinates()
    }

    /// Calculate the length of the line
    pub fn length(&self) -> f64 {
        let from = self.from_coordinates();
        let to = self.to_coordinates();
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        let dz = to.z - from.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Create from indices within a system order
    pub fn from_indices(from: Index, to: Index, order: u8) -> Option<Self> {
        let from_point = Point::from_index(from, order)?;
        let to_point = Point::from_index(to, order)?;
        Self::new(from_point, to_point)
    }

    /// Get all lines for a complete graph of given order
    pub fn all_for_order(order: u8) -> Vec<Line> {
        let points = Point::all_for_order(order);
        let mut lines = Vec::new();
        for (i, from) in points.iter().enumerate() {
            for to in points.iter().skip(i + 1) {
                if let Some(line) = Line::new(*from, *to) {
                    lines.push(line);
                }
            }
        }
        lines
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Line({} -> {}, len={:.3})",
            self.from.position(),
            self.to.position(),
            self.length()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let point = Point::new(3, 2).unwrap();
        assert_eq!(point.order(), 3);
        assert_eq!(point.position(), 2);
        assert!(matches!(point, Point::Triadic(TriadicPoint::Two)));
    }

    #[test]
    fn test_point_invalid() {
        assert!(Point::new(3, 4).is_none()); // Position exceeds order
        assert!(Point::new(0, 1).is_none()); // Invalid order
        assert!(Point::new(13, 1).is_none()); // Order too high
        assert!(Point::new(3, 0).is_none()); // Position 0 invalid
    }

    #[test]
    fn test_point_to_index() {
        let point = Point::new(5, 3).unwrap();
        let index = point.to_index();
        assert_eq!(index, Index::Three);
    }

    #[test]
    fn test_point_coordinates() {
        // Monad should be at origin
        let point = Point::new(1, 1).unwrap();
        let coords = point.coordinates();
        assert_eq!(coords.x, 0.0);
        assert_eq!(coords.y, 0.0);
        assert_eq!(coords.z, 0.0);
    }

    #[test]
    fn test_all_points_for_order() {
        let points = Point::all_for_order(3);
        assert_eq!(points.len(), 3);
        assert_eq!(points[0].position(), 1);
        assert_eq!(points[1].position(), 2);
        assert_eq!(points[2].position(), 3);
    }

    #[test]
    fn test_line_creation() {
        let from = Point::new(3, 1).unwrap();
        let to = Point::new(3, 2).unwrap();
        let line = Line::new(from, to).unwrap();
        assert_eq!(line.order(), 3);
        assert!(line.length() > 0.0);
    }

    #[test]
    fn test_line_invalid() {
        let point1 = Point::new(3, 1).unwrap();
        let point2 = Point::new(4, 1).unwrap(); // Different order
        assert!(Line::new(point1, point2).is_none());

        let point3 = Point::new(3, 1).unwrap();
        assert!(Line::new(point1, point3).is_none()); // Same position
    }

    #[test]
    fn test_all_lines_for_order() {
        // Triad: 3 points, 3 lines (complete graph K3)
        let lines = Line::all_for_order(3);
        assert_eq!(lines.len(), 3);

        // Tetrad: 4 points, 6 lines (complete graph K4)
        let lines = Line::all_for_order(4);
        assert_eq!(lines.len(), 6);
    }

    #[test]
    fn test_sub_enum_discriminants() {
        assert_eq!(TriadicPoint::One as u8, 1);
        assert_eq!(TriadicPoint::Two as u8, 2);
        assert_eq!(TriadicPoint::Three as u8, 3);

        assert_eq!(DodecadicPoint::Twelve as u8, 12);
    }
}
