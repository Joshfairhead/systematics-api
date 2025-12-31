//! GraphQL types and schema for the Systematics property graph API.

use async_graphql::*;
use crate::core::{
    Graph, Entry, Link, LinkType, Language,
    Character, Term, Coordinate, Colour,
    SystemName, CoherenceAttribute, TermDesignation, ConnectiveDesignation,
};
use crate::data;

/// Root query object
#[derive(Clone, Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // ========================================================================
    // Graph Queries
    // ========================================================================

    /// Get the full graph with all entries and links
    async fn graph(&self) -> GqlGraph {
        GqlGraph::new(data::build_graph())
    }

    // ========================================================================
    // System Queries
    // ========================================================================

    /// Get system by order (1-12)
    async fn system(&self, order: i32) -> Option<GqlSystemView> {
        if order < 1 || order > 12 {
            return None;
        }
        let graph = data::build_graph();
        Some(GqlSystemView::new(order as u8, graph))
    }

    /// Get all systems (1-12)
    async fn all_systems(&self) -> Vec<GqlSystemView> {
        let graph = data::build_graph();
        (1..=12)
            .map(|order| GqlSystemView::new(order, graph.clone()))
            .collect()
    }

    /// Get system by name (e.g., "Triad")
    async fn system_by_name(&self, name: String) -> Option<GqlSystemView> {
        let order = match name.to_lowercase().as_str() {
            "monad" => 1,
            "dyad" => 2,
            "triad" => 3,
            "tetrad" => 4,
            "pentad" => 5,
            "hexad" => 6,
            "heptad" => 7,
            "octad" => 8,
            "ennead" => 9,
            "decad" => 10,
            "undecad" => 11,
            "dodecad" => 12,
            _ => return None,
        };
        let graph = data::build_graph();
        Some(GqlSystemView::new(order, graph))
    }

    // ========================================================================
    // Term Queries
    // ========================================================================

    /// Get term at a specific order and position
    async fn term(&self, order: i32, position: i32) -> Option<GqlTerm> {
        let graph = data::build_graph();
        graph.term(order as u8, position as u8).map(|t| GqlTerm::new(t.clone(), &graph))
    }

    /// Get all terms for an order
    async fn terms(&self, order: i32, language: Option<GqlLanguage>) -> Vec<GqlTerm> {
        let graph = data::build_graph();
        let lang = language.map(|l| l.into());
        graph.terms(order as u8, lang)
            .into_iter()
            .map(|t| GqlTerm::new(t.clone(), &graph))
            .collect()
    }

    // ========================================================================
    // Character Queries
    // ========================================================================

    /// Get all characters for a language
    async fn characters(&self, language: GqlLanguage) -> Vec<GqlCharacter> {
        let graph = data::build_graph();
        graph.characters(language.into())
            .into_iter()
            .map(|c| GqlCharacter::new(c.clone()))
            .collect()
    }

    // ========================================================================
    // Slice Queries
    // ========================================================================

    /// Get slice (all entries at order+position)
    async fn slice(&self, order: i32, position: i32) -> GqlSlice {
        let graph = data::build_graph();
        GqlSlice::new(order as u8, position as u8, graph)
    }

    // ========================================================================
    // Language Queries
    // ========================================================================

    /// Get all available languages
    async fn languages(&self) -> Vec<GqlLanguage> {
        vec![
            GqlLanguage::Canonical,
            GqlLanguage::Energy,
            GqlLanguage::Values,
            GqlLanguage::Society,
            GqlLanguage::Hex,
            GqlLanguage::Name,
        ]
    }

    /// Get vocabulary languages (for Character entries)
    async fn vocabulary_languages(&self) -> Vec<GqlLanguage> {
        vec![
            GqlLanguage::Canonical,
            GqlLanguage::Energy,
            GqlLanguage::Values,
            GqlLanguage::Society,
        ]
    }
}

// ============================================================================
// GraphQL Enums
// ============================================================================

/// Language enum for vocabularies and representations
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum GqlLanguage {
    Canonical,
    Energy,
    Values,
    Society,
    Hex,
    Name,
}

impl From<GqlLanguage> for Language {
    fn from(l: GqlLanguage) -> Self {
        match l {
            GqlLanguage::Canonical => Language::Canonical,
            GqlLanguage::Energy => Language::Energy,
            GqlLanguage::Values => Language::Values,
            GqlLanguage::Society => Language::Society,
            GqlLanguage::Hex => Language::Hex,
            GqlLanguage::Name => Language::Name,
        }
    }
}

impl From<Language> for GqlLanguage {
    fn from(l: Language) -> Self {
        match l {
            Language::Canonical => GqlLanguage::Canonical,
            Language::Energy => GqlLanguage::Energy,
            Language::Values => GqlLanguage::Values,
            Language::Society => GqlLanguage::Society,
            Language::Hex => GqlLanguage::Hex,
            Language::Name => GqlLanguage::Name,
        }
    }
}

/// Link type enum
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum GqlLinkType {
    Line,
    Edge,
    Connective,
}

// ============================================================================
// Graph Types
// ============================================================================

/// The full property graph
pub struct GqlGraph {
    graph: Graph,
}

impl GqlGraph {
    pub fn new(graph: Graph) -> Self {
        Self { graph }
    }
}

#[Object]
impl GqlGraph {
    /// Total number of entries
    async fn entry_count(&self) -> i32 {
        self.graph.entries.len() as i32
    }

    /// Total number of links
    async fn link_count(&self) -> i32 {
        self.graph.links.len() as i32
    }

    /// All entries in the graph
    async fn entries(&self) -> Vec<GqlEntry> {
        self.graph.entries.iter().map(|e| GqlEntry::new(e.clone(), &self.graph)).collect()
    }

    /// All links in the graph
    async fn links(&self) -> Vec<GqlLink> {
        self.graph.links.iter().map(|l| GqlLink::new(l.clone(), &self.graph)).collect()
    }

    /// Get entry by ID
    async fn entry(&self, id: String) -> Option<GqlEntry> {
        self.graph.get_entry(&id).map(|e| GqlEntry::new(e.clone(), &self.graph))
    }

    /// Get link by ID
    async fn link(&self, id: String) -> Option<GqlLink> {
        self.graph.get_link(&id).map(|l| GqlLink::new(l.clone(), &self.graph))
    }
}

// ============================================================================
// Entry Types
// ============================================================================

/// A graph entry (union type)
pub struct GqlEntry {
    entry: Entry,
    graph: Graph,
}

impl GqlEntry {
    pub fn new(entry: Entry, graph: &Graph) -> Self {
        Self { entry, graph: graph.clone() }
    }
}

#[Object]
impl GqlEntry {
    /// Entry ID
    async fn id(&self) -> &str {
        self.entry.id()
    }

    /// Entry order (if applicable)
    async fn order(&self) -> Option<i32> {
        self.entry.order().map(|o| o as i32)
    }

    /// Entry position (if applicable)
    async fn position(&self) -> Option<i32> {
        self.entry.position().map(|p| p as i32)
    }

    /// Is this a system-level entry?
    async fn is_system_level(&self) -> bool {
        self.entry.is_system_level()
    }

    /// Is this a positional entry?
    async fn is_positional(&self) -> bool {
        self.entry.is_positional()
    }

    /// Entry type name
    async fn entry_type(&self) -> &str {
        match &self.entry {
            Entry::SystemName(_) => "SystemName",
            Entry::CoherenceAttribute(_) => "CoherenceAttribute",
            Entry::TermDesignation(_) => "TermDesignation",
            Entry::ConnectiveDesignation(_) => "ConnectiveDesignation",
            Entry::Term(_) => "Term",
            Entry::Colour(_) => "Colour",
            Entry::Coordinate(_) => "Coordinate",
            Entry::Character(_) => "Character",
        }
    }

    /// As SystemName (if applicable)
    async fn as_system_name(&self) -> Option<GqlSystemName> {
        match &self.entry {
            Entry::SystemName(s) => Some(GqlSystemName::new(s.clone())),
            _ => None,
        }
    }

    /// As CoherenceAttribute (if applicable)
    async fn as_coherence(&self) -> Option<GqlCoherenceAttribute> {
        match &self.entry {
            Entry::CoherenceAttribute(c) => Some(GqlCoherenceAttribute::new(c.clone())),
            _ => None,
        }
    }

    /// As TermDesignation (if applicable)
    async fn as_term_designation(&self) -> Option<GqlTermDesignation> {
        match &self.entry {
            Entry::TermDesignation(t) => Some(GqlTermDesignation::new(t.clone())),
            _ => None,
        }
    }

    /// As ConnectiveDesignation (if applicable)
    async fn as_connective_designation(&self) -> Option<GqlConnectiveDesignation> {
        match &self.entry {
            Entry::ConnectiveDesignation(c) => Some(GqlConnectiveDesignation::new(c.clone())),
            _ => None,
        }
    }

    /// As Term (if applicable)
    async fn as_term(&self) -> Option<GqlTerm> {
        match &self.entry {
            Entry::Term(t) => Some(GqlTerm::new(t.clone(), &self.graph)),
            _ => None,
        }
    }

    /// As Colour (if applicable)
    async fn as_colour(&self) -> Option<GqlColour> {
        match &self.entry {
            Entry::Colour(c) => Some(GqlColour::new(c.clone())),
            _ => None,
        }
    }

    /// As Coordinate (if applicable)
    async fn as_coordinate(&self) -> Option<GqlCoordinate> {
        match &self.entry {
            Entry::Coordinate(c) => Some(GqlCoordinate::new(c.clone())),
            _ => None,
        }
    }

    /// As Character (if applicable)
    async fn as_character(&self) -> Option<GqlCharacter> {
        match &self.entry {
            Entry::Character(c) => Some(GqlCharacter::new(c.clone())),
            _ => None,
        }
    }
}

// ============================================================================
// Link Type
// ============================================================================

/// A link between entries
pub struct GqlLink {
    link: Link,
    graph: Graph,
}

impl GqlLink {
    pub fn new(link: Link, graph: &Graph) -> Self {
        Self { link, graph: graph.clone() }
    }
}

#[Object]
impl GqlLink {
    /// Link ID
    async fn id(&self) -> &str {
        &self.link.id
    }

    /// Base (source) entry ID
    async fn base_id(&self) -> &str {
        &self.link.base
    }

    /// Target entry ID
    async fn target_id(&self) -> &str {
        &self.link.target
    }

    /// Link type
    async fn link_type(&self) -> GqlLinkType {
        match &self.link.link_type {
            LinkType::Line => GqlLinkType::Line,
            LinkType::Edge => GqlLinkType::Edge,
            LinkType::Connective(_) => GqlLinkType::Connective,
        }
    }

    /// Character ID (for connective links)
    async fn character_id(&self) -> Option<&str> {
        self.link.character_id()
    }

    /// Optional tag
    async fn tag(&self) -> Option<&str> {
        self.link.tag.as_deref()
    }

    /// Base entry
    async fn base(&self) -> Option<GqlEntry> {
        self.graph.get_entry(&self.link.base).map(|e| GqlEntry::new(e.clone(), &self.graph))
    }

    /// Target entry
    async fn target(&self) -> Option<GqlEntry> {
        self.graph.get_entry(&self.link.target).map(|e| GqlEntry::new(e.clone(), &self.graph))
    }

    /// Character (for connective links)
    async fn character(&self) -> Option<GqlCharacter> {
        self.link.character_id()
            .and_then(|id| self.graph.get_character(id))
            .map(|c| GqlCharacter::new(c.clone()))
    }
}

// ============================================================================
// Specific Entry Types
// ============================================================================

/// Character entry
pub struct GqlCharacter {
    character: Character,
}

impl GqlCharacter {
    pub fn new(character: Character) -> Self {
        Self { character }
    }
}

#[Object]
impl GqlCharacter {
    async fn id(&self) -> &str {
        &self.character.id
    }

    async fn language(&self) -> GqlLanguage {
        self.character.language.into()
    }

    async fn value(&self) -> &str {
        &self.character.value
    }
}

/// Term entry
pub struct GqlTerm {
    term: Term,
    graph: Graph,
}

impl GqlTerm {
    pub fn new(term: Term, graph: &Graph) -> Self {
        Self { term, graph: graph.clone() }
    }
}

#[Object]
impl GqlTerm {
    async fn id(&self) -> &str {
        &self.term.id
    }

    async fn order(&self) -> i32 {
        self.term.order as i32
    }

    async fn position(&self) -> i32 {
        self.term.position as i32
    }

    async fn character_id(&self) -> &str {
        &self.term.character
    }

    /// The character this term references
    async fn character(&self) -> Option<GqlCharacter> {
        self.graph.get_character(&self.term.character).map(|c| GqlCharacter::new(c.clone()))
    }

    /// Connectives involving this term
    async fn connectives(&self) -> Vec<GqlLink> {
        self.graph.connectives_for_term(&self.term.id)
            .into_iter()
            .map(|l| GqlLink::new(l.clone(), &self.graph))
            .collect()
    }
}

/// Coordinate entry
pub struct GqlCoordinate {
    coordinate: Coordinate,
}

impl GqlCoordinate {
    pub fn new(coordinate: Coordinate) -> Self {
        Self { coordinate }
    }
}

#[Object]
impl GqlCoordinate {
    async fn id(&self) -> &str {
        &self.coordinate.id
    }

    async fn order(&self) -> i32 {
        self.coordinate.order as i32
    }

    async fn position(&self) -> i32 {
        self.coordinate.position as i32
    }

    async fn x(&self) -> f64 {
        self.coordinate.value.x
    }

    async fn y(&self) -> f64 {
        self.coordinate.value.y
    }

    async fn z(&self) -> f64 {
        self.coordinate.value.z
    }
}

/// Colour entry
pub struct GqlColour {
    colour: Colour,
}

impl GqlColour {
    pub fn new(colour: Colour) -> Self {
        Self { colour }
    }
}

#[Object]
impl GqlColour {
    async fn id(&self) -> &str {
        &self.colour.id
    }

    async fn order(&self) -> i32 {
        self.colour.order as i32
    }

    async fn position(&self) -> i32 {
        self.colour.position as i32
    }

    async fn language(&self) -> GqlLanguage {
        self.colour.language.into()
    }

    async fn value(&self) -> &str {
        &self.colour.value
    }
}

/// SystemName entry
pub struct GqlSystemName {
    system_name: SystemName,
}

impl GqlSystemName {
    pub fn new(system_name: SystemName) -> Self {
        Self { system_name }
    }
}

#[Object]
impl GqlSystemName {
    async fn id(&self) -> &str {
        &self.system_name.id
    }

    async fn order(&self) -> i32 {
        self.system_name.order as i32
    }

    async fn value(&self) -> &str {
        &self.system_name.value
    }
}

/// CoherenceAttribute entry
pub struct GqlCoherenceAttribute {
    coherence: CoherenceAttribute,
}

impl GqlCoherenceAttribute {
    pub fn new(coherence: CoherenceAttribute) -> Self {
        Self { coherence }
    }
}

#[Object]
impl GqlCoherenceAttribute {
    async fn id(&self) -> &str {
        &self.coherence.id
    }

    async fn order(&self) -> i32 {
        self.coherence.order as i32
    }

    async fn value(&self) -> &str {
        &self.coherence.value
    }
}

/// TermDesignation entry
pub struct GqlTermDesignation {
    term_designation: TermDesignation,
}

impl GqlTermDesignation {
    pub fn new(term_designation: TermDesignation) -> Self {
        Self { term_designation }
    }
}

#[Object]
impl GqlTermDesignation {
    async fn id(&self) -> &str {
        &self.term_designation.id
    }

    async fn order(&self) -> i32 {
        self.term_designation.order as i32
    }

    async fn value(&self) -> &str {
        &self.term_designation.value
    }
}

/// ConnectiveDesignation entry
pub struct GqlConnectiveDesignation {
    connective_designation: ConnectiveDesignation,
}

impl GqlConnectiveDesignation {
    pub fn new(connective_designation: ConnectiveDesignation) -> Self {
        Self { connective_designation }
    }
}

#[Object]
impl GqlConnectiveDesignation {
    async fn id(&self) -> &str {
        &self.connective_designation.id
    }

    async fn order(&self) -> i32 {
        self.connective_designation.order as i32
    }

    async fn value(&self) -> &str {
        &self.connective_designation.value
    }
}

// ============================================================================
// System View
// ============================================================================

/// A view of a system at a given order
pub struct GqlSystemView {
    order: u8,
    graph: Graph,
}

impl GqlSystemView {
    pub fn new(order: u8, graph: Graph) -> Self {
        Self { order, graph }
    }
}

#[Object]
impl GqlSystemView {
    async fn order(&self) -> i32 {
        self.order as i32
    }

    async fn name(&self) -> Option<String> {
        self.graph.system_name(self.order).map(|s| s.value.clone())
    }

    async fn coherence(&self) -> Option<String> {
        self.graph.coherence(self.order).map(|c| c.value.clone())
    }

    async fn term_designation(&self) -> Option<String> {
        self.graph.term_designation(self.order).map(|t| t.value.clone())
    }

    async fn connective_designation(&self) -> Option<String> {
        self.graph.connective_designation(self.order).map(|c| c.value.clone())
    }

    async fn terms(&self) -> Vec<GqlTerm> {
        self.graph.terms(self.order, None)
            .into_iter()
            .map(|t| GqlTerm::new(t.clone(), &self.graph))
            .collect()
    }

    async fn coordinates(&self) -> Vec<GqlCoordinate> {
        self.graph.coordinates(self.order)
            .into_iter()
            .map(|c| GqlCoordinate::new(c.clone()))
            .collect()
    }

    async fn colours(&self) -> Vec<GqlColour> {
        self.graph.colours(self.order)
            .into_iter()
            .map(|c| GqlColour::new(c.clone()))
            .collect()
    }

    async fn connectives(&self) -> Vec<GqlLink> {
        self.graph.connectives(self.order, None, None)
            .into_iter()
            .map(|l| GqlLink::new(l.clone(), &self.graph))
            .collect()
    }

    async fn lines(&self) -> Vec<GqlLink> {
        self.graph.lines(self.order)
            .into_iter()
            .map(|l| GqlLink::new(l.clone(), &self.graph))
            .collect()
    }

    /// Get slice at a specific position
    async fn slice(&self, position: i32) -> GqlSlice {
        GqlSlice::new(self.order, position as u8, self.graph.clone())
    }

    /// All slices for this system
    async fn slices(&self) -> Vec<GqlSlice> {
        (1..=self.order)
            .map(|pos| GqlSlice::new(self.order, pos, self.graph.clone()))
            .collect()
    }
}

// ============================================================================
// Slice View
// ============================================================================

/// A slice - all entries at a specific order+position
pub struct GqlSlice {
    order: u8,
    position: u8,
    graph: Graph,
}

impl GqlSlice {
    pub fn new(order: u8, position: u8, graph: Graph) -> Self {
        Self { order, position, graph }
    }
}

#[Object]
impl GqlSlice {
    async fn order(&self) -> i32 {
        self.order as i32
    }

    async fn position(&self) -> i32 {
        self.position as i32
    }

    async fn entries(&self) -> Vec<GqlEntry> {
        self.graph.slice(self.order, self.position)
            .into_iter()
            .map(|e| GqlEntry::new(e.clone(), &self.graph))
            .collect()
    }

    async fn term(&self) -> Option<GqlTerm> {
        self.graph.term(self.order, self.position)
            .map(|t| GqlTerm::new(t.clone(), &self.graph))
    }

    async fn coordinate(&self) -> Option<GqlCoordinate> {
        self.graph.coordinate(self.order, self.position)
            .map(|c| GqlCoordinate::new(c.clone()))
    }

    async fn colour(&self, language: Option<GqlLanguage>) -> Option<GqlColour> {
        let lang = language.map(|l| l.into()).unwrap_or(Language::Hex);
        self.graph.colour(self.order, self.position, lang)
            .map(|c| GqlColour::new(c.clone()))
    }

    /// All isomorphic terms at this position (across languages)
    async fn isomorphic_terms(&self) -> Vec<GqlTerm> {
        self.graph.isomorphic_terms(self.order, self.position)
            .into_iter()
            .map(|(t, _)| GqlTerm::new(t.clone(), &self.graph))
            .collect()
    }
}

// ============================================================================
// Schema
// ============================================================================

pub type SystematicsSchema = async_graphql::Schema<QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

pub fn create_schema() -> SystematicsSchema {
    async_graphql::Schema::build(QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription)
        .finish()
}
