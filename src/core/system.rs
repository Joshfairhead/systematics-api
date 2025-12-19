use crate::core::fiber::Fiber;
use crate::core::system_content::SystemContent;
use crate::core::system_topology::SystemTopology;
use crate::core::universal_index::UniversalIndex;
use std::fmt;

/// Error types for System operations
#[derive(Debug, Clone, PartialEq)]
pub enum SystemError {
    InvalidOrder(String),
    InvalidIndices(String),
    InvalidCoordinates(String),
    InvalidFiberCount(String),
    TopologyMismatch(String),
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SystemError::InvalidOrder(msg) => write!(f, "Invalid order: {}", msg),
            SystemError::InvalidIndices(msg) => write!(f, "Invalid indices: {}", msg),
            SystemError::InvalidCoordinates(msg) => write!(f, "Invalid coordinates: {}", msg),
            SystemError::InvalidFiberCount(msg) => write!(f, "Invalid fiber count: {}", msg),
            SystemError::TopologyMismatch(msg) => write!(f, "Topology mismatch: {}", msg),
        }
    }
}

impl std::error::Error for SystemError {}

/// System<T> is the container holding fibers and connectivity
#[derive(Debug, Clone)]
pub struct System<T: SystemContent> {
    fibers: Vec<Fiber<T>>,
    connectivity: Vec<Vec<bool>>,
}

impl<T: SystemContent> System<T> {
    /// Get the order of this system (number of fibers)
    pub fn order(&self) -> u8 {
        self.fibers.len() as u8
    }

    /// Get the fibers
    pub fn fibers(&self) -> &[Fiber<T>] {
        &self.fibers
    }

    /// Get the connectivity matrix
    pub fn connectivity(&self) -> &[Vec<bool>] {
        &self.connectivity
    }

    /// Check if two indices are connected
    pub fn is_connected(&self, from: UniversalIndex, to: UniversalIndex) -> Result<bool, SystemError> {
        let from_idx = from.to_zero_based();
        let to_idx = to.to_zero_based();

        if from_idx >= self.connectivity.len() || to_idx >= self.connectivity[0].len() {
            return Err(SystemError::InvalidIndices(
                "Index out of bounds".to_string(),
            ));
        }

        Ok(self.connectivity[from_idx][to_idx])
    }

    /// Get all connections for a given index
    pub fn connections(&self, index: UniversalIndex) -> Result<Vec<UniversalIndex>, SystemError> {
        let idx = index.to_zero_based();

        if idx >= self.connectivity.len() {
            return Err(SystemError::InvalidIndices(
                "Index out of bounds".to_string(),
            ));
        }

        let mut connections = Vec::new();
        for (i, &connected) in self.connectivity[idx].iter().enumerate() {
            if connected {
                if let Ok(target_index) = UniversalIndex::from_zero_based(i) {
                    connections.push(target_index);
                }
            }
        }

        Ok(connections)
    }

    // ==================== METHOD A: TOP-DOWN FACTORY ====================

    /// Generate a System from scratch using a content provider
    /// This is the factory pattern: "Create a Triad using Colors"
    pub fn generate(order: u8, content_provider: T) -> Result<Self, SystemError> {
        // Validate order
        if !(1..=12).contains(&order) {
            return Err(SystemError::InvalidOrder(
                format!("Order must be 1-12, got {}", order),
            ));
        }

        // Get canonical coordinates for this order
        let coordinates = SystemTopology::get_coordinates(order)
            .map_err(|e| SystemError::InvalidOrder(e.to_string()))?;

        // Get content items from provider
        let content_items = content_provider.provide(order as usize);

        if content_items.len() != order as usize {
            return Err(SystemError::InvalidFiberCount(
                format!("Expected {} items, got {}", order, content_items.len()),
            ));
        }

        // Create fibers
        let mut fibers = Vec::new();
        for i in 0..order {
            let index = UniversalIndex::new(i + 1)
                .map_err(|e| SystemError::InvalidIndices(e.to_string()))?;

            let fiber = Fiber::new(
                index,
                coordinates[i as usize],
                content_items[i as usize].clone(),
            );

            fibers.push(fiber);
        }

        // Generate connectivity matrix (complete graph by default)
        let connectivity = SystemTopology::get_adjacency_matrix(order);

        Ok(System {
            fibers,
            connectivity,
        })
    }

    // ==================== METHOD B: BOTTOM-UP ASSEMBLY ====================

    /// Assemble a System from raw fibers (inference pattern)
    /// This validates and infers the System from components
    pub fn try_assemble(fibers: Vec<Fiber<T>>) -> Result<Self, SystemError> {
        // Validate fiber count
        let count = fibers.len();
        if count == 0 || count > 12 {
            return Err(SystemError::InvalidFiberCount(
                format!("Fiber count must be 1-12, got {}", count),
            ));
        }

        let order = count as u8;

        // Validate indices are correct (1, 2, 3, ..., order)
        let mut indices: Vec<u8> = fibers.iter().map(|f| f.index().value()).collect();
        indices.sort_unstable();

        let expected_indices: Vec<u8> = (1..=order).collect();
        if indices != expected_indices {
            return Err(SystemError::InvalidIndices(
                format!("Expected indices {:?}, got {:?}", expected_indices, indices),
            ));
        }

        // Validate coordinates match topology
        let canonical_coords = SystemTopology::get_coordinates(order)
            .map_err(|e| SystemError::InvalidOrder(e.to_string()))?;

        for fiber in &fibers {
            let index = fiber.index();
            let expected_coord = canonical_coords[index.to_zero_based()];
            let actual_coord = fiber.coordinates();

            // Allow small floating-point tolerance
            let tolerance = 1e-6;
            if (actual_coord.x - expected_coord.x).abs() > tolerance
                || (actual_coord.y - expected_coord.y).abs() > tolerance
                || (actual_coord.z - expected_coord.z).abs() > tolerance
            {
                return Err(SystemError::InvalidCoordinates(format!(
                    "Fiber {} has incorrect coordinates: expected ({:.3}, {:.3}, {:.3}), got ({:.3}, {:.3}, {:.3})",
                    index.value(),
                    expected_coord.x,
                    expected_coord.y,
                    expected_coord.z,
                    actual_coord.x,
                    actual_coord.y,
                    actual_coord.z
                )));
            }
        }

        // Sort fibers by index to ensure correct ordering
        let mut sorted_fibers = fibers;
        sorted_fibers.sort_by_key(|f| f.index().value());

        // Infer connectivity matrix (default to complete graph)
        let connectivity = SystemTopology::get_adjacency_matrix(order);

        Ok(System {
            fibers: sorted_fibers,
            connectivity,
        })
    }

    /// Custom connectivity: assemble with a custom adjacency matrix
    pub fn try_assemble_with_connectivity(
        fibers: Vec<Fiber<T>>,
        connectivity: Vec<Vec<bool>>,
    ) -> Result<Self, SystemError> {
        // First validate as normal
        let mut system = Self::try_assemble(fibers)?;

        // Validate connectivity matrix dimensions
        let order = system.order() as usize;
        if connectivity.len() != order || connectivity.iter().any(|row| row.len() != order) {
            return Err(SystemError::TopologyMismatch(
                format!("Connectivity matrix must be {}x{}", order, order),
            ));
        }

        // Replace default connectivity with custom
        system.connectivity = connectivity;

        Ok(system)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::system_content::Color;
    use crate::core::system_topology::Point3d;

    #[test]
    fn test_generate_triad() {
        let system = System::generate(3, Color::Red).unwrap();

        assert_eq!(system.order(), 3);
        assert_eq!(system.fibers().len(), 3);
        assert_eq!(system.connectivity().len(), 3);

        // Check that fibers have correct content
        assert_eq!(*system.fibers()[0].content(), Color::Red);
        assert_eq!(*system.fibers()[1].content(), Color::Green);
        assert_eq!(*system.fibers()[2].content(), Color::Blue);
    }

    #[test]
    fn test_generate_invalid_order() {
        let result = System::generate(0, Color::Red);
        assert!(result.is_err());

        let result = System::generate(13, Color::Red);
        assert!(result.is_err());
    }

    #[test]
    fn test_try_assemble_valid() {
        // Create fibers manually
        let coords = SystemTopology::get_coordinates(2).unwrap();

        let fiber1 = Fiber::new(
            UniversalIndex::new(1).unwrap(),
            coords[0],
            String::from("First"),
        );
        let fiber2 = Fiber::new(
            UniversalIndex::new(2).unwrap(),
            coords[1],
            String::from("Second"),
        );

        let system = System::try_assemble(vec![fiber1, fiber2]).unwrap();

        assert_eq!(system.order(), 2);
        assert_eq!(system.fibers().len(), 2);
    }

    #[test]
    fn test_try_assemble_invalid_indices() {
        let coords = SystemTopology::get_coordinates(2).unwrap();

        // Missing index 1, has 2 and 3
        let fiber2 = Fiber::new(
            UniversalIndex::new(2).unwrap(),
            coords[0],
            String::from("Second"),
        );
        let fiber3 = Fiber::new(
            UniversalIndex::new(3).unwrap(),
            coords[1],
            String::from("Third"),
        );

        let result = System::try_assemble(vec![fiber2, fiber3]);
        assert!(result.is_err());
    }

    #[test]
    fn test_try_assemble_invalid_coordinates() {
        let fiber1 = Fiber::new(
            UniversalIndex::new(1).unwrap(),
            Point3d::new(0.0, 0.0, 0.0), // Wrong coordinate
            String::from("First"),
        );
        let fiber2 = Fiber::new(
            UniversalIndex::new(2).unwrap(),
            Point3d::new(999.0, 999.0, 0.0), // Wrong coordinate
            String::from("Second"),
        );

        let result = System::try_assemble(vec![fiber1, fiber2]);
        assert!(result.is_err());
    }

    #[test]
    fn test_connectivity_queries() {
        let system = System::generate(3, Color::Red).unwrap();

        let idx1 = UniversalIndex::new(1).unwrap();
        let idx2 = UniversalIndex::new(2).unwrap();

        // In complete graph, all nodes are connected
        assert!(system.is_connected(idx1, idx2).unwrap());

        let connections = system.connections(idx1).unwrap();
        assert_eq!(connections.len(), 2); // Connected to 2 and 3
    }
}
