use crate::core::topology::Node;

pub struct K8Topology;

impl K8Topology {
    pub const NODES: [Node; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
}
