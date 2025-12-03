use crate::core::topology::{Node, Edge};
use crate::core::geometry::Coordinates;

pub struct TetradSystem;

impl TetradSystem {
    // Metadata
    pub const SYSTEM_NAME: &'static str = "Tetrad";
    pub const COHERENCE_ATTRIBUTE: &'static str = "Activity Field";
    pub const TERM_DESIGNATION: &'static str = "Sources";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Interplays";
    pub const SOURCE: &'static str = "Elementary Systematics";

    // Vocabulary
    pub const TERM_CHARACTERS: [&'static str; 4] = ["Ideal", "Directive", "Instrumental", "Ground"];
    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 6] = [
        ("Receptive Regard", "Ideal", "Directive"),
        ("Effectual Compatibility", "Ideal", "Instrumental"),
        ("Motivational Imperative", "Ideal", "Ground"),
        ("Demonstrable Activity", "Directive", "Instrumental"),
        ("Material Mastery", "Directive", "Ground"),
        ("Technical Power", "Instrumental", "Ground"),
    ];

    // Topology
    pub const NODES: [Node; 4] = [0, 1, 2, 3];
    pub const EDGES: [Edge; 6] = [
        (0, 1),  // ideal-directive
        (0, 2),  // ideal-instrumental
        (0, 3),  // ideal-ground
        (1, 2),  // directive-instrumental
        (1, 3),  // directive-ground
        (2, 3),  // instrumental-ground
    ];

    // Geometry
    pub const POINTS: [Coordinates; 4] = [
        Coordinates { x: 0.0, y: 1.0, z: None },   // ideal
        Coordinates { x: 1.0, y: 0.0, z: None },   // directive
        Coordinates { x: -1.0, y: 0.0, z: None },  // instrumental
        Coordinates { x: 0.0, y: -1.0, z: None },  // ground
    ];
    pub const LINES: [(Coordinates, Coordinates); 6] = [
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: 1.0, y: 0.0, z: None }),  // ideal-directive
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: -1.0, y: 0.0, z: None }),  // ideal-instrumental
        (Coordinates { x: 0.0, y: 1.0, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }),  // ideal-ground
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: -1.0, y: 0.0, z: None }),  // directive-instrumental
        (Coordinates { x: 1.0, y: 0.0, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }),  // directive-ground
        (Coordinates { x: -1.0, y: 0.0, z: None }, Coordinates { x: 0.0, y: -1.0, z: None }),  // instrumental-ground
    ];
}
