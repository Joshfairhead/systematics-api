use crate::core::state_manager::Index;

pub struct K2Topology;

impl K2Topology {
    pub const INDEX: [Index; 2] = [0, 1];
    pub const INDICES: [(Index, Index); 1] = [
        (0, 1), // Single edge
    ];
}
