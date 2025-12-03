use crate::core::topology::{Node, Edge};

pub struct K3Topology;

impl K3Topology {
    pub const NODES: [Node; 3] = [0, 1, 2];
    pub const EDGES: [Edge; 3] = [
        (0, 1),  // Will-Function (vertical left)
        (1, 2),  // Function-Being (bottom right)
        (2, 0),  // Being-Will (top right)
    ];
}
