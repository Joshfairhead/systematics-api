use crate::core::topology::Node;

pub struct K4Topology;

impl K4Topology {
    pub const NODES: [Node; 4] = [0, 1, 2, 3];
}
