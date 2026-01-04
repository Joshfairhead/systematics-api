//! Core types for the Systematics property graph.
//!
//! This module provides the fundamental building blocks:
//! - `Language` - Semantic vocabularies and representation types
//! - `entries` - Entry types (Character, Term, Coordinate, Colour, etc.) and the Entry enum
//! - `links` - Link types (Line, Connective)
//! - `graph` - Graph structure with query methods

pub mod entries;
pub mod graph;
pub mod language;
pub mod links;

// Re-export language types
pub use language::Language;

// Re-export entry types (including Entry enum)
pub use entries::{
    Character, CoherenceAttribute, ConnectiveDesignation, Colour, Coordinate, Entry, Point3d,
    SystemName, Term, TermDesignation,
};

// Re-export link types
pub use links::{Link, LinkType};

// Re-export graph types
pub use graph::Graph;
