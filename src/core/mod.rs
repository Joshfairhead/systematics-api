// Core reference types (re-exported at module root for flat access)
mod index;
mod color;
mod registry;
mod node;
mod point;
mod mapping;

pub use index::Index;
pub use color::Color;
pub use registry::{Registry, CANONICAL};
pub use node::{Node, Edge, MonadicNode, DyadicNode, TriadicNode, TetradicNode,
               PentadicNode, HexadicNode, HeptadicNode, OctadicNode,
               EnneadicNode, DecadicNode, UndecadicNode, DodecadicNode};
pub use point::{Point, Line, MonadicPoint, DyadicPoint, TriadicPoint, TetradicPoint,
                PentadicPoint, HexadicPoint, HeptadicPoint, OctadicPoint,
                EnneadicPoint, DecadicPoint, UndecadicPoint, DodecadicPoint};
pub use mapping::Mapping;

// Supporting modules (legacy - to be refactored)
pub mod topology;  // TODO: Migrate to node.rs
pub mod geometry;  // TODO: Migrate to point.rs
pub mod essence;
pub mod system_topology;
pub mod system_content;
pub mod fiber;
pub mod system;
