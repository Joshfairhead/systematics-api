//! Link types for the property graph.
//!
//! Links are explicit relationships between entries.
//! They connect entries via base (source) and target IDs.

use serde::{Deserialize, Serialize};

/// LinkType defines the kind of relationship between entries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LinkType {
    /// Line connects Coordinate → Coordinate (geometric edge)
    Line,
    /// Connective connects Term → Term, referencing a Character entry
    Connective(String), // Character ID
}

/// Link is an explicit relationship between two entries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub id: String,
    /// Entry ID of the source
    pub base: String,
    /// Entry ID of the target
    pub target: String,
    /// Type of the link
    pub link_type: LinkType,
    /// Optional payload/tag
    pub tag: Option<String>,
}

impl Link {
    pub fn new(
        id: impl Into<String>,
        base: impl Into<String>,
        target: impl Into<String>,
        link_type: LinkType,
    ) -> Self {
        Self {
            id: id.into(),
            base: base.into(),
            target: target.into(),
            link_type,
            tag: None,
        }
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// Create a Line link between two coordinates
    pub fn line(base: impl Into<String>, target: impl Into<String>) -> Self {
        let base = base.into();
        let target = target.into();
        let id = format!("line_{}_{}", base, target);
        Self::new(id, base, target, LinkType::Line)
    }

    /// Create a Connective link between two terms, referencing a Character
    pub fn connective(
        base: impl Into<String>,
        target: impl Into<String>,
        character_id: impl Into<String>,
    ) -> Self {
        let base = base.into();
        let target = target.into();
        let character_id = character_id.into();
        let id = format!("conn_{}_{}_{}", base, target, character_id);
        Self::new(id, base, target, LinkType::Connective(character_id))
    }

    /// Check if this is a connective link
    pub fn is_connective(&self) -> bool {
        matches!(self.link_type, LinkType::Connective(_))
    }

    /// Get the character ID if this is a connective link
    pub fn character_id(&self) -> Option<&str> {
        match &self.link_type {
            LinkType::Connective(id) => Some(id),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_link() {
        let link = Link::line("coord_3_1", "coord_3_2");
        assert_eq!(link.base, "coord_3_1");
        assert_eq!(link.target, "coord_3_2");
        assert!(matches!(link.link_type, LinkType::Line));
    }

    #[test]
    fn test_connective_link() {
        let link = Link::connective("term_3_1", "term_3_2", "char_act1");
        assert!(link.is_connective());
        assert_eq!(link.character_id(), Some("char_act1"));
    }

    #[test]
    fn test_link_with_tag() {
        let link = Link::line("a", "b").with_tag("my_tag");
        assert_eq!(link.tag, Some("my_tag".to_string()));
    }
}
