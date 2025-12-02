use crate::core::state_manager::Index;

pub struct K6Topology;

impl K6Topology {
    pub const INDEX: [Index; 6] = [0, 1, 2, 3, 4, 5];
    pub const INDICES: [(Index, Index); 15] = [
        (0, 1), (0, 2), (0, 3), (0, 4), (0, 5),
        (1, 2), (1, 3), (1, 4), (1, 5),
        (2, 3), (2, 4), (2, 5),
        (3, 4), (3, 5),
        (4, 5),
    ];
}
