use crate::core::topology::Node;

pub struct K3Topology;

impl K3Topology {
    pub const NODES: [Node; 3] = [0, 1, 2];
}
