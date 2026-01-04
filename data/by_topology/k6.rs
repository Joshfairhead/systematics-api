use crate::core::topology::Node;

pub struct K6Topology;

impl K6Topology {
    pub const NODES: [Node; 6] = [0, 1, 2, 3, 4, 5];
}
