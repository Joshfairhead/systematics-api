//! Core types for the Systematics property graph.
//!
//! This module provides the fundamental building blocks:
//! - `Language` - Semantic vocabularies and representation types
//! - `entries` - Entry types (Character, Term, Coordinate, Colour, etc.)
//! - `links` - Link types (Line, Edge, Connective)
//! - `graph` - Graph structure with Entry enum and queries

pub mod language;
pub mod entries;
pub mod links;
pub mod graph;

// Re-export language types
pub use language::Language;

// Re-export entry types
pub use entries::{
    Character, CoherenceAttribute, ConnectiveDesignation, Colour, Coordinate, Point3d,
    SystemName, Term, TermDesignation,
};

// Re-export link types
pub use links::{Link, LinkType};

// Re-export graph types
pub use graph::{Entry, Graph};
