use crate::core::topology::{Node, Edge};

pub struct K2Topology;

impl K2Topology {
    pub const NODES: [Node; 2] = [0, 1];
    pub const EDGES: [Edge; 1] = [
        (0, 1), // Single edge
    ];
}
