use crate::core::state_manager::Index;

pub struct K5Topology;

impl K5Topology {
    pub const INDEX: [Index; 5] = [0, 1, 2, 3, 4];
    pub const INDICES: [(Index, Index); 10] = [
        (0, 1), // Purpose-Higher Potential
        (1, 2), // Higher Potential-Quintessence
        (2, 3), // Quintessence-Lower Potential
        (3, 4), // Lower Potential-Source
        (4, 0), // Source-Purpose
        (0, 2), // Purpose-Quintessence
        (1, 3), // Higher Potential-Lower Potential
        (2, 4), // Quintessence-Source
        (3, 0), // Lower Potential-Purpose
        (4, 1), // Source-Higher Potential
    ];
}
