use crate::core::topology::{Node, Edge};
use crate::core::geometry::Coordinates;

pub struct MonadSystem;

impl MonadSystem {
    // Metadata
    pub const SYSTEM_NAME: &'static str = "Monad";
    pub const COHERENCE_ATTRIBUTE: &'static str = "Universality";
    pub const TERM_DESIGNATION: &'static str = "Totality";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Unity";
    pub const SOURCE: &'static str = "Elementary Systematics";

    // Vocabulary
    pub const TERM_CHARACTERS: [&'static str; 1] = ["Unity"];
    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 0] = [];

    // Topology
    pub const NODES: [Node; 1] = [0];
    pub const EDGES: [Edge; 0] = [];

    // Geometry
    pub const POINTS: [Coordinates; 1] = [
        Coordinates { x: 0.0, y: 0.0, z: None }, // Single point
    ];
    pub const LINES: [(Coordinates, Coordinates); 0] = [];
}
