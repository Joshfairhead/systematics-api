use crate::core::topology::Node;

pub struct K5Topology;

impl K5Topology {
    pub const NODES: [Node; 5] = [0, 1, 2, 3, 4];
}
