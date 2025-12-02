use crate::core::state_manager::Index;

pub struct K4Topology;

impl K4Topology {
    pub const INDEX: [Index; 4] = [0, 1, 2, 3];
    pub const INDICES: [(Index, Index); 6] = [
        (0, 1),  // ideal-directive
        (0, 2),  // ideal-instrumental
        (0, 3),  // ideal-ground
        (1, 2),  // directive-instrumental
        (1, 3),  // directive-ground
        (2, 3),  // instrumental-ground
    ];
}
