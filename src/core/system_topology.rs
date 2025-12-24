use crate::core::geometry::Coordinates;
use crate::core::Index;

/// Point3d represents a 3D coordinate
#[derive(Debug, Clone, Copy, PartialEq)]
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

impl From<Point3d> for Coordinates {
    fn from(point: Point3d) -> Self {
        Coordinates {
            x: point.x,
            y: point.y,
            z: Some(point.z),
        }
    }
}

/// SystemTopology provides canonical coordinates for each system order
pub struct SystemTopology;

impl SystemTopology {
    /// Get canonical coordinates for a given system order
    /// Returns a vector of Point3d coordinates
    pub fn get_coordinates(order: u8) -> Result<Vec<Point3d>, &'static str> {
        match order {
            1 => Ok(Self::monad_coordinates()),
            2 => Ok(Self::dyad_coordinates()),
            3 => Ok(Self::triad_coordinates()),
            4 => Ok(Self::tetrad_coordinates()),
            5 => Ok(Self::pentad_coordinates()),
            6 => Ok(Self::hexad_coordinates()),
            7 => Ok(Self::heptad_coordinates()),
            8 => Ok(Self::octad_coordinates()),
            9 => Ok(Self::ennead_coordinates()),
            10 => Ok(Self::decad_coordinates()),
            11 => Ok(Self::undecad_coordinates()),
            12 => Ok(Self::dodecad_coordinates()),
            _ => Err("Order must be between 1 and 12"),
        }
    }

    /// Get coordinate for a specific index within a system
    pub fn get_coordinate(index: Index, system_order: u8) -> Result<Point3d, &'static str> {
        let coords = Self::get_coordinates(system_order)?;
        let zero_based = index.to_zero_based();

        if zero_based < coords.len() {
            Ok(coords[zero_based])
        } else {
            Err("Index exceeds system order")
        }
    }

    fn monad_coordinates() -> Vec<Point3d> {
        vec![Point3d::new(0.0, 0.0, 0.0)]
    }

    fn dyad_coordinates() -> Vec<Point3d> {
        vec![
            Point3d::new(-1.0, 0.0, 0.0), // Essence
            Point3d::new(1.0, 0.0, 0.0),  // Existence
        ]
    }

    fn triad_coordinates() -> Vec<Point3d> {
        use std::f64::consts::PI;
        vec![
            Point3d::new(0.0, -1.0, 0.0),                    // Function (bottom)
            Point3d::new((2.0 * PI / 3.0).cos(), (2.0 * PI / 3.0).sin(), 0.0), // Being (upper left)
            Point3d::new((4.0 * PI / 3.0).cos(), (4.0 * PI / 3.0).sin(), 0.0), // Will (upper right)
        ]
    }

    fn tetrad_coordinates() -> Vec<Point3d> {
        vec![
            Point3d::new(0.0, 1.0, 0.0),   // Ideal (top)
            Point3d::new(1.0, 0.0, 0.0),   // Directive (right)
            Point3d::new(0.0, -1.0, 0.0),  // Instrumental (bottom)
            Point3d::new(-1.0, 0.0, 0.0),  // Ground (left)
        ]
    }

    fn pentad_coordinates() -> Vec<Point3d> {
        use std::f64::consts::PI;
        let angle_offset = -PI / 2.0; // Start from top
        (0..5)
            .map(|i| {
                let angle = angle_offset + (i as f64 * 2.0 * PI / 5.0);
                Point3d::new(angle.cos(), angle.sin(), 0.0)
            })
            .collect()
    }

    fn hexad_coordinates() -> Vec<Point3d> {
        use std::f64::consts::PI;
        let angle_offset = PI / 2.0; // Start from top
        (0..6)
            .map(|i| {
                let angle = angle_offset - (i as f64 * 2.0 * PI / 6.0);
                Point3d::new(angle.cos(), angle.sin(), 0.0)
            })
            .collect()
    }

    fn heptad_coordinates() -> Vec<Point3d> {
        use std::f64::consts::PI;
        let angle_offset = -PI / 2.0;
        (0..7)
            .map(|i| {
                let angle = angle_offset + (i as f64 * 2.0 * PI / 7.0);
                Point3d::new(angle.cos(), angle.sin(), 0.0)
            })
            .collect()
    }

    fn octad_coordinates() -> Vec<Point3d> {
        use std::f64::consts::PI;
        let angle_offset = -PI / 2.0;
        (0..8)
            .map(|i| {
                let angle = angle_offset + (i as f64 * 2.0 * PI / 8.0);
                Point3d::new(angle.cos(), angle.sin(), 0.0)
            })
            .collect()
    }

    fn ennead_coordinates() -> Vec<Point3d> {
        use std::f64::consts::PI;
        let angle_offset = -PI / 2.0;
        (0..9)
            .map(|i| {
                let angle = angle_offset + (i as f64 * 2.0 * PI / 9.0);
                Point3d::new(angle.cos(), angle.sin(), 0.0)
            })
            .collect()
    }

    fn decad_coordinates() -> Vec<Point3d> {
        use std::f64::consts::PI;
        let angle_offset = -PI / 2.0;
        (0..10)
            .map(|i| {
                let angle = angle_offset + (i as f64 * 2.0 * PI / 10.0);
                Point3d::new(angle.cos(), angle.sin(), 0.0)
            })
            .collect()
    }

    fn undecad_coordinates() -> Vec<Point3d> {
        use std::f64::consts::PI;
        let angle_offset = -PI / 2.0;
        (0..11)
            .map(|i| {
                let angle = angle_offset + (i as f64 * 2.0 * PI / 11.0);
                Point3d::new(angle.cos(), angle.sin(), 0.0)
            })
            .collect()
    }

    fn dodecad_coordinates() -> Vec<Point3d> {
        use std::f64::consts::PI;
        let angle_offset = -PI / 2.0;
        (0..12)
            .map(|i| {
                let angle = angle_offset + (i as f64 * 2.0 * PI / 12.0);
                Point3d::new(angle.cos(), angle.sin(), 0.0)
            })
            .collect()
    }

    /// Generate adjacency matrix (complete graph) for a given order
    pub fn get_adjacency_matrix(order: u8) -> Vec<Vec<bool>> {
        let n = order as usize;
        let mut matrix = vec![vec![false; n]; n];

        // Complete graph: all nodes connected to all others
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    matrix[i][j] = true;
                }
            }
        }

        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_counts() {
        assert_eq!(SystemTopology::get_coordinates(1).unwrap().len(), 1);
        assert_eq!(SystemTopology::get_coordinates(2).unwrap().len(), 2);
        assert_eq!(SystemTopology::get_coordinates(3).unwrap().len(), 3);
        assert_eq!(SystemTopology::get_coordinates(12).unwrap().len(), 12);
    }

    #[test]
    fn test_monad_at_origin() {
        let coords = SystemTopology::get_coordinates(1).unwrap();
        assert_eq!(coords[0].x, 0.0);
        assert_eq!(coords[0].y, 0.0);
        assert_eq!(coords[0].z, 0.0);
    }

    #[test]
    fn test_adjacency_matrix() {
        let matrix = SystemTopology::get_adjacency_matrix(3);
        assert_eq!(matrix.len(), 3);
        assert_eq!(matrix[0].len(), 3);
        assert!(!matrix[0][0]); // No self-loops
        assert!(matrix[0][1]);  // Connected to others
        assert!(matrix[1][2]);
    }
}
