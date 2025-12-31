//! Graph structure with Entry enum and query methods.
//!
//! The Graph is the primary container for the property graph,
//! holding all entries and links with methods for querying.

use serde::{Deserialize, Serialize};

use super::entries::{
    Character, CoherenceAttribute, ConnectiveDesignation, Colour, Coordinate, SystemName, Term,
    TermDesignation,
};
use super::language::Language;
use super::links::{Link, LinkType};

/// Entry is a sum type for storing heterogeneous entries in a single collection.
/// This enables the Graph to hold all entry types in one `entries` field,
/// allowing unified iteration and queries across all entry kinds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Entry {
    // System-level (per-order, no position)
    SystemName(SystemName),
    CoherenceAttribute(CoherenceAttribute),
    TermDesignation(TermDesignation),
    ConnectiveDesignation(ConnectiveDesignation),

    // Positional entries (per order+position)
    Term(Term),
    Colour(Colour),
    Coordinate(Coordinate),

    // Semantic content (reusable expressions)
    Character(Character),
}

impl Entry {
    /// Get the ID of this entry
    pub fn id(&self) -> &str {
        match self {
            Entry::SystemName(e) => &e.id,
            Entry::CoherenceAttribute(e) => &e.id,
            Entry::TermDesignation(e) => &e.id,
            Entry::ConnectiveDesignation(e) => &e.id,
            Entry::Term(e) => &e.id,
            Entry::Colour(e) => &e.id,
            Entry::Coordinate(e) => &e.id,
            Entry::Character(e) => &e.id,
        }
    }

    /// Get the order of this entry (if applicable)
    pub fn order(&self) -> Option<u8> {
        match self {
            Entry::SystemName(e) => Some(e.order),
            Entry::CoherenceAttribute(e) => Some(e.order),
            Entry::TermDesignation(e) => Some(e.order),
            Entry::ConnectiveDesignation(e) => Some(e.order),
            Entry::Term(e) => Some(e.order),
            Entry::Colour(e) => Some(e.order),
            Entry::Coordinate(e) => Some(e.order),
            Entry::Character(_) => None, // Characters are order-independent
        }
    }

    /// Get the position of this entry (if applicable)
    pub fn position(&self) -> Option<u8> {
        match self {
            Entry::Term(e) => Some(e.position),
            Entry::Colour(e) => Some(e.position),
            Entry::Coordinate(e) => Some(e.position),
            _ => None, // System-level entries and Characters don't have positions
        }
    }

    /// Check if this entry is system-level (per-order, no position)
    pub fn is_system_level(&self) -> bool {
        matches!(
            self,
            Entry::SystemName(_)
                | Entry::CoherenceAttribute(_)
                | Entry::TermDesignation(_)
                | Entry::ConnectiveDesignation(_)
        )
    }

    /// Check if this entry is positional (has order and position)
    pub fn is_positional(&self) -> bool {
        matches!(self, Entry::Term(_) | Entry::Colour(_) | Entry::Coordinate(_))
    }
}

/// Graph is the primary container for the property graph (AD4M: Perspective).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Graph {
    pub entries: Vec<Entry>,
    pub links: Vec<Link>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an entry to the graph
    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    /// Add a link to the graph
    pub fn add_link(&mut self, link: Link) {
        self.links.push(link);
    }

    /// Find an entry by ID
    pub fn get_entry(&self, id: &str) -> Option<&Entry> {
        self.entries.iter().find(|e| e.id() == id)
    }

    /// Find a link by ID
    pub fn get_link(&self, id: &str) -> Option<&Link> {
        self.links.iter().find(|l| l.id == id)
    }

    // ==================== System-level queries ====================

    /// Get all entries for a given order
    pub fn system(&self, order: u8) -> Vec<&Entry> {
        self.entries
            .iter()
            .filter(|e| e.order() == Some(order))
            .collect()
    }

    /// Get the system name for an order
    pub fn system_name(&self, order: u8) -> Option<&SystemName> {
        self.entries.iter().find_map(|e| match e {
            Entry::SystemName(s) if s.order == order => Some(s),
            _ => None,
        })
    }

    /// Get the coherence attribute for an order
    pub fn coherence(&self, order: u8) -> Option<&CoherenceAttribute> {
        self.entries.iter().find_map(|e| match e {
            Entry::CoherenceAttribute(c) if c.order == order => Some(c),
            _ => None,
        })
    }

    /// Get the term designation for an order
    pub fn term_designation(&self, order: u8) -> Option<&TermDesignation> {
        self.entries.iter().find_map(|e| match e {
            Entry::TermDesignation(t) if t.order == order => Some(t),
            _ => None,
        })
    }

    /// Get the connective designation for an order
    pub fn connective_designation(&self, order: u8) -> Option<&ConnectiveDesignation> {
        self.entries.iter().find_map(|e| match e {
            Entry::ConnectiveDesignation(c) if c.order == order => Some(c),
            _ => None,
        })
    }

    // ==================== Term queries ====================

    /// Get all terms for an order, optionally filtered by language of their character
    pub fn terms(&self, order: u8, language: Option<Language>) -> Vec<&Term> {
        let terms: Vec<&Term> = self
            .entries
            .iter()
            .filter_map(|e| match e {
                Entry::Term(t) if t.order == order => Some(t),
                _ => None,
            })
            .collect();

        if let Some(lang) = language {
            terms
                .into_iter()
                .filter(|t| {
                    self.get_character(&t.character)
                        .map(|c| c.language == lang)
                        .unwrap_or(false)
                })
                .collect()
        } else {
            terms
        }
    }

    /// Get a specific term by order and position
    pub fn term(&self, order: u8, position: u8) -> Option<&Term> {
        self.entries.iter().find_map(|e| match e {
            Entry::Term(t) if t.order == order && t.position == position => Some(t),
            _ => None,
        })
    }

    // ==================== Character queries ====================

    /// Get all characters for a language
    pub fn characters(&self, language: Language) -> Vec<&Character> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Character(c) if c.language == language => Some(c),
                _ => None,
            })
            .collect()
    }

    /// Get a character by ID
    pub fn get_character(&self, id: &str) -> Option<&Character> {
        self.entries.iter().find_map(|e| match e {
            Entry::Character(c) if c.id == id => Some(c),
            _ => None,
        })
    }

    // ==================== Coordinate queries ====================

    /// Get all coordinates for an order
    pub fn coordinates(&self, order: u8) -> Vec<&Coordinate> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Coordinate(c) if c.order == order => Some(c),
                _ => None,
            })
            .collect()
    }

    /// Get a specific coordinate by order and position
    pub fn coordinate(&self, order: u8, position: u8) -> Option<&Coordinate> {
        self.entries.iter().find_map(|e| match e {
            Entry::Coordinate(c) if c.order == order && c.position == position => Some(c),
            _ => None,
        })
    }

    // ==================== Colour queries ====================

    /// Get all colours for an order
    pub fn colours(&self, order: u8) -> Vec<&Colour> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Colour(c) if c.order == order => Some(c),
                _ => None,
            })
            .collect()
    }

    /// Get a specific colour by order, position, and language
    pub fn colour(&self, order: u8, position: u8, language: Language) -> Option<&Colour> {
        self.entries.iter().find_map(|e| match e {
            Entry::Colour(c)
                if c.order == order && c.position == position && c.language == language =>
            {
                Some(c)
            }
            _ => None,
        })
    }

    // ==================== Link queries ====================

    /// Get connective links, optionally filtered by order and/or base/target positions
    pub fn connectives(
        &self,
        order: u8,
        base_position: Option<u8>,
        target_position: Option<u8>,
    ) -> Vec<&Link> {
        self.links
            .iter()
            .filter(|l| {
                if !l.is_connective() {
                    return false;
                }

                // Get the terms for base and target
                let base_term = self.entries.iter().find_map(|e| match e {
                    Entry::Term(t) if t.id == l.base => Some(t),
                    _ => None,
                });
                let target_term = self.entries.iter().find_map(|e| match e {
                    Entry::Term(t) if t.id == l.target => Some(t),
                    _ => None,
                });

                // Both terms must exist and be in the specified order
                match (base_term, target_term) {
                    (Some(bt), Some(tt)) if bt.order == order && tt.order == order => {
                        let base_match = base_position.map(|p| bt.position == p).unwrap_or(true);
                        let target_match =
                            target_position.map(|p| tt.position == p).unwrap_or(true);
                        base_match && target_match
                    }
                    _ => false,
                }
            })
            .collect()
    }

    /// Get all connectives involving a specific term
    pub fn connectives_for_term(&self, term_id: &str) -> Vec<&Link> {
        self.links
            .iter()
            .filter(|l| l.is_connective() && (l.base == term_id || l.target == term_id))
            .collect()
    }

    /// Get all line links for an order
    pub fn lines(&self, order: u8) -> Vec<&Link> {
        self.links
            .iter()
            .filter(|l| {
                if !matches!(l.link_type, LinkType::Line) {
                    return false;
                }

                // Check that base coordinate is in the specified order
                self.entries.iter().any(|e| match e {
                    Entry::Coordinate(c) if c.id == l.base && c.order == order => true,
                    _ => false,
                })
            })
            .collect()
    }

    /// Get all edge links for an order
    pub fn edges(&self, order: u8) -> Vec<&Link> {
        self.links
            .iter()
            .filter(|l| {
                if !matches!(l.link_type, LinkType::Edge) {
                    return false;
                }

                // Check that base term is in the specified order
                self.entries.iter().any(|e| match e {
                    Entry::Term(t) if t.id == l.base && t.order == order => true,
                    _ => false,
                })
            })
            .collect()
    }

    // ==================== Slice query ====================

    /// Get all entries at a specific order+position (the "slice")
    pub fn slice(&self, order: u8, position: u8) -> Vec<&Entry> {
        self.entries
            .iter()
            .filter(|e| e.order() == Some(order) && e.position() == Some(position))
            .collect()
    }

    // ==================== Cross-language queries ====================

    /// Get all terms at the same position across different languages
    pub fn isomorphic_terms(&self, order: u8, position: u8) -> Vec<(&Term, &Character)> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Term(t) if t.order == order && t.position == position => {
                    self.get_character(&t.character).map(|c| (t, c))
                }
                _ => None,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::entries::Point3d;

    fn create_test_graph() -> Graph {
        let mut graph = Graph::new();

        // Add system-level metadata
        graph.add_entry(Entry::SystemName(SystemName::with_auto_id(3, "Triad")));
        graph.add_entry(Entry::CoherenceAttribute(CoherenceAttribute::with_auto_id(
            3, "Dynamism",
        )));
        graph.add_entry(Entry::TermDesignation(TermDesignation::with_auto_id(
            3, "Impulses",
        )));
        graph.add_entry(Entry::ConnectiveDesignation(
            ConnectiveDesignation::with_auto_id(3, "Acts"),
        ));

        // Add characters
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            "Will",
        )));
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            "Function",
        )));
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            "Being",
        )));

        // Add terms
        graph.add_entry(Entry::Term(Term::with_auto_id(3, 1, "char_canonical_will")));
        graph.add_entry(Entry::Term(Term::with_auto_id(
            3,
            2,
            "char_canonical_function",
        )));
        graph.add_entry(Entry::Term(Term::with_auto_id(
            3,
            3,
            "char_canonical_being",
        )));

        // Add coordinates
        graph.add_entry(Entry::Coordinate(Coordinate::with_auto_id(
            3,
            1,
            Point3d::new(0.0, 1.0, 0.0),
        )));
        graph.add_entry(Entry::Coordinate(Coordinate::with_auto_id(
            3,
            2,
            Point3d::new(-0.866, -0.5, 0.0),
        )));
        graph.add_entry(Entry::Coordinate(Coordinate::with_auto_id(
            3,
            3,
            Point3d::new(0.866, -0.5, 0.0),
        )));

        // Add colours
        graph.add_entry(Entry::Colour(Colour::with_auto_id(
            3,
            1,
            Language::Hex,
            "#FF0000",
        )));
        graph.add_entry(Entry::Colour(Colour::with_auto_id(
            3,
            2,
            Language::Hex,
            "#0000FF",
        )));
        graph.add_entry(Entry::Colour(Colour::with_auto_id(
            3,
            3,
            Language::Hex,
            "#E6E600",
        )));

        graph
    }

    #[test]
    fn test_system_queries() {
        let graph = create_test_graph();

        assert!(graph.system_name(3).is_some());
        assert_eq!(graph.system_name(3).unwrap().value, "Triad");

        assert!(graph.coherence(3).is_some());
        assert_eq!(graph.coherence(3).unwrap().value, "Dynamism");

        assert!(graph.term_designation(3).is_some());
        assert_eq!(graph.term_designation(3).unwrap().value, "Impulses");
    }

    #[test]
    fn test_term_queries() {
        let graph = create_test_graph();

        let terms = graph.terms(3, None);
        assert_eq!(terms.len(), 3);

        let term = graph.term(3, 1);
        assert!(term.is_some());
        assert_eq!(term.unwrap().character, "char_canonical_will");
    }

    #[test]
    fn test_slice_query() {
        let graph = create_test_graph();

        let slice = graph.slice(3, 1);
        // Should contain: Term, Coordinate, Colour at position 1
        assert_eq!(slice.len(), 3);
    }

    #[test]
    fn test_character_lookup() {
        let graph = create_test_graph();

        let char = graph.get_character("char_canonical_will");
        assert!(char.is_some());
        assert_eq!(char.unwrap().value, "Will");
    }
}
