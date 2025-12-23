// Core reference types (re-exported at module root for flat access)
mod index;
mod color;
mod registry;

pub use index::Index;
pub use color::Color;
pub use registry::{Registry, CANONICAL};

// Supporting modules
pub mod topology;
pub mod geometry;
pub mod essence;
pub mod system_topology;
pub mod system_content;
pub mod fiber;
pub mod system;
