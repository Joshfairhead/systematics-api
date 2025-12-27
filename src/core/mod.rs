// Core reference types (re-exported at module root for flat access)
mod index;
mod color;
mod registry;
mod node;
mod point;
mod mapping;

// New category-theoretic modules
pub mod category;
pub mod entry;
pub mod transforms;
pub mod views;
pub mod config;
pub mod link;
pub mod connective;
pub mod generators;

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

// Re-export new types for convenient access
pub use category::CategoryType;
pub use entry::{Entry, GeometricEntry, LexiconEntry};
pub use views::{SimplexView, FiberView, SystemView, MetadataType};
pub use config::{FiberConfig, SystemConfig, DEFAULT_CATEGORIES, ALL_CATEGORIES};
pub use link::{Link, LinkInfo};
pub use connective::Connective;
pub use generators::{generate_simplex, generate_fiber, generate_fiber_with, generate_system, generate_system_with};

// Supporting modules (legacy - to be refactored)
pub mod topology;  // TODO: Migrate to node.rs
pub mod geometry;  // TODO: Migrate to point.rs
pub mod essence;
pub mod system_topology;
pub mod system_content;
pub mod fiber;
pub mod system;
