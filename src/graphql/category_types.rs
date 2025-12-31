//! GraphQL types for the category-theoretic layer.
//!
//! Exposes the new generator-based API:
//! - SimplexView: The Kn complete graph scaffold
//! - FiberView: Bundle of entries at a position
//! - SystemView: Complete generated system
//! - Entry types for each category

use async_graphql::*;
use crate::core::{
    CategoryType, Index, Color,
    views::{SimplexView as CoreSimplexView, FiberView as CoreFiberView, SystemView as CoreSystemView, MetadataType},
    entry::{Entry as CoreEntry},
    transforms::to_connective_characters,
};

// ============================================================================
// Enums
// ============================================================================

/// Category types in the system
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum GqlCategoryType {
    /// Foundational reference (Index enum)
    Index,
    /// Combinatorial structure (Node, Edge)
    Topological,
    /// Spatial structure (Point, Line, coordinates)
    Geometric,
    /// Semantic structure (Term, Connective) from a vocabulary
    Lexicon,
    /// Chromatic (Color)
    Color,
}

impl From<CategoryType> for GqlCategoryType {
    fn from(ct: CategoryType) -> Self {
        match ct {
            CategoryType::Index => GqlCategoryType::Index,
            CategoryType::Topological => GqlCategoryType::Topological,
            CategoryType::Geometric => GqlCategoryType::Geometric,
            CategoryType::Lexicon => GqlCategoryType::Lexicon,
            CategoryType::Color => GqlCategoryType::Color,
        }
    }
}

impl From<GqlCategoryType> for CategoryType {
    fn from(gct: GqlCategoryType) -> Self {
        match gct {
            GqlCategoryType::Index => CategoryType::Index,
            GqlCategoryType::Topological => CategoryType::Topological,
            GqlCategoryType::Geometric => CategoryType::Geometric,
            GqlCategoryType::Lexicon => CategoryType::Lexicon,
            GqlCategoryType::Color => CategoryType::Color,
        }
    }
}

/// Metadata types for system generation
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum GqlMetadataType {
    /// System name (Monad, Dyad, Triad, etc.)
    Name,
    /// Coherence attribute (Unity, Complementarity, Dynamism, etc.)
    Coherence,
    /// Term designation (Totality, Poles, Impulses, etc.)
    TermDesignation,
    /// Connective designation (Unity, Force, Acts, etc.)
    ConnectiveDesignation,
}

impl From<MetadataType> for GqlMetadataType {
    fn from(mt: MetadataType) -> Self {
        match mt {
            MetadataType::Name => GqlMetadataType::Name,
            MetadataType::Coherence => GqlMetadataType::Coherence,
            MetadataType::TermDesignation => GqlMetadataType::TermDesignation,
            MetadataType::ConnectiveDesignation => GqlMetadataType::ConnectiveDesignation,
        }
    }
}

impl From<GqlMetadataType> for MetadataType {
    fn from(gmt: GqlMetadataType) -> Self {
        match gmt {
            GqlMetadataType::Name => MetadataType::Name,
            GqlMetadataType::Coherence => MetadataType::Coherence,
            GqlMetadataType::TermDesignation => MetadataType::TermDesignation,
            GqlMetadataType::ConnectiveDesignation => MetadataType::ConnectiveDesignation,
        }
    }
}

// ============================================================================
// Input Types
// ============================================================================

/// Configuration for fiber generation
#[derive(InputObject, Debug)]
pub struct FiberConfigInput {
    /// Categories to include (defaults to Index, Geometric, Vocabulary)
    pub categories: Option<Vec<GqlCategoryType>>,
}

/// Configuration for system generation
#[derive(InputObject, Debug)]
pub struct SystemConfigInput {
    /// Categories to include in fibers (defaults to Index, Geometric, Vocabulary)
    pub categories: Option<Vec<GqlCategoryType>>,
    /// Metadata to include (defaults to Name, Coherence)
    pub include_metadata: Option<Vec<GqlMetadataType>>,
}

// ============================================================================
// Index Info Type
// ============================================================================

/// Information about an Index value
#[derive(Clone, Debug)]
pub struct IndexInfo {
    index: Index,
}

impl IndexInfo {
    pub fn new(index: Index) -> Self {
        Self { index }
    }
}

#[Object]
impl IndexInfo {
    /// The numeric value (1-12)
    async fn value(&self) -> i32 {
        self.index.value() as i32
    }

    /// The delta representation (0-indexed for simplex category)
    async fn delta(&self) -> i32 {
        self.index.to_delta() as i32
    }

    /// The system name for this order
    async fn system_name(&self) -> &str {
        crate::core::transforms::to_system_name(self.index)
    }

    /// The coherence attribute for this order
    async fn coherence(&self) -> &str {
        crate::core::transforms::to_coherence(self.index)
    }

    /// The term designation (if available)
    async fn term_designation(&self) -> Option<&str> {
        crate::core::transforms::to_term_designation(self.index)
    }

    /// The connective designation (if available)
    async fn connective_designation(&self) -> Option<&str> {
        crate::core::transforms::to_connective_designation(self.index)
    }

    /// Number of vertices in Kn
    async fn vertex_count(&self) -> i32 {
        crate::core::transforms::vertex_count(self.index) as i32
    }

    /// Number of edges in complete graph Kn = n(n-1)/2
    async fn edge_count(&self) -> i32 {
        crate::core::transforms::edge_count(self.index) as i32
    }
}

// ============================================================================
// Color Info Type
// ============================================================================

/// Information about a Color value
#[derive(Clone, Debug)]
pub struct ColorInfo {
    color: Color,
}

impl ColorInfo {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

#[Object]
impl ColorInfo {
    /// The numeric value (1-12)
    async fn value(&self) -> i32 {
        self.color.value() as i32
    }

    /// The canonical color name
    async fn name(&self) -> &str {
        self.color.canonical_name()
    }

    /// The hex color code
    async fn hex(&self) -> &str {
        self.color.canonical_hex()
    }

    /// The associated Index
    async fn index(&self) -> i32 {
        self.color.to_index().value() as i32
    }
}

// ============================================================================
// Entry Types
// ============================================================================

/// An entry in a fiber (0-simplex in any category)
#[derive(Clone, Debug)]
pub struct GqlEntry {
    entry: CoreEntry,
}

#[Object]
impl GqlEntry {
    /// The category this entry belongs to
    async fn category(&self) -> GqlCategoryType {
        self.entry.category().into()
    }

    /// The simplex dimension (always 0 for entries)
    async fn dimension(&self) -> i32 {
        0
    }

    /// As Index value (if Index category)
    async fn as_index(&self) -> Option<i32> {
        self.entry.as_index().map(|i| i.value() as i32)
    }

    /// As Node info (if Topological category)
    async fn as_node(&self) -> Option<NodeInfo> {
        self.entry.as_node().map(|n| NodeInfo {
            order: n.order(),
            position: n.position(),
        })
    }

    /// As Geometric info (if Geometric category)
    async fn as_geometric(&self) -> Option<GeometricInfo> {
        self.entry.as_geometric().map(|g| GeometricInfo {
            order: g.point.order(),
            position: g.point.position(),
            x: g.coordinates.x,
            y: g.coordinates.y,
            z: g.coordinates.z,
        })
    }

    /// As Lexicon info (if Lexicon category)
    async fn as_lexicon(&self) -> Option<LexiconInfo> {
        self.entry.as_lexicon().map(|l| LexiconInfo {
            vocabulary: "canonical".to_string(),
            term: l.term.clone(),
            description: l.description.clone(),
        })
    }

    /// As Color info (if Color category)
    async fn as_color(&self) -> Option<ColorInfo> {
        self.entry.as_color().map(|c| ColorInfo { color: c })
    }
}

/// Node information (topological entry)
#[derive(Clone, Debug, SimpleObject)]
pub struct NodeInfo {
    /// System order this node belongs to
    pub order: u8,
    /// Position within the system (1-indexed)
    pub position: u8,
}

/// Geometric entry information
#[derive(Clone, Debug, SimpleObject)]
pub struct GeometricInfo {
    /// System order
    pub order: u8,
    /// Position within the system (1-indexed)
    pub position: u8,
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Z coordinate
    pub z: f64,
}

/// Lexicon entry information (term from a vocabulary)
#[derive(Clone, Debug, SimpleObject)]
pub struct LexiconInfo {
    /// The vocabulary reference (e.g., "canonical")
    pub vocabulary: String,
    /// The term name
    pub term: String,
    /// Optional description
    pub description: Option<String>,
}

/// Connective information (1-simplex in lexicon category)
#[derive(Clone, Debug, SimpleObject)]
pub struct ConnectiveInfo {
    /// The vocabulary reference (e.g., "canonical")
    pub vocabulary: String,
    /// The connective name
    pub name: String,
    /// The source term
    pub from: String,
    /// The target term
    pub to: String,
}

// ============================================================================
// Simplex View
// ============================================================================

/// The simplex scaffold (Kn complete graph)
#[derive(Clone, Debug)]
pub struct GqlSimplexView {
    inner: CoreSimplexView,
}

impl GqlSimplexView {
    pub fn new(inner: CoreSimplexView) -> Self {
        Self { inner }
    }
}

#[Object]
impl GqlSimplexView {
    /// The order of this simplex
    async fn order(&self) -> i32 {
        self.inner.order.value() as i32
    }

    /// Number of vertices
    async fn vertex_count(&self) -> i32 {
        self.inner.vertex_count() as i32
    }

    /// Number of edges
    async fn edge_count(&self) -> i32 {
        self.inner.edge_count() as i32
    }

    /// The vertex indices (1-indexed)
    async fn vertices(&self) -> Vec<i32> {
        self.inner.vertices.iter().map(|i| i.value() as i32).collect()
    }

    /// The edges as pairs of indices
    async fn edges(&self) -> Vec<EdgePair> {
        self.inner.edges.iter().map(|(from, to)| EdgePair {
            from: from.value() as i32,
            to: to.value() as i32,
        }).collect()
    }
}

/// An edge pair (from, to)
#[derive(Clone, Debug, SimpleObject)]
pub struct EdgePair {
    /// Source vertex (1-indexed)
    pub from: i32,
    /// Target vertex (1-indexed)
    pub to: i32,
}

// ============================================================================
// Fiber View
// ============================================================================

/// A fiber - bundle of entries at a position across categories
#[derive(Clone, Debug)]
pub struct GqlFiberView {
    inner: CoreFiberView,
}

impl GqlFiberView {
    pub fn new(inner: CoreFiberView) -> Self {
        Self { inner }
    }
}

#[Object]
impl GqlFiberView {
    /// The system order this fiber belongs to
    async fn order(&self) -> i32 {
        self.inner.order.value() as i32
    }

    /// The position within the system (1-indexed)
    async fn position(&self) -> i32 {
        self.inner.position.value() as i32
    }

    /// Number of categories included
    async fn category_count(&self) -> i32 {
        self.inner.category_count() as i32
    }

    /// The categories included in this fiber
    async fn categories(&self) -> Vec<GqlCategoryType> {
        self.inner.entries.keys().map(|c| (*c).into()).collect()
    }

    /// All entries in this fiber
    async fn entries(&self) -> Vec<GqlEntry> {
        self.inner.entries.values()
            .map(|e| GqlEntry { entry: e.clone() })
            .collect()
    }

    /// Get entry for a specific category
    async fn entry(&self, category: GqlCategoryType) -> Option<GqlEntry> {
        self.inner.get(category.into()).map(|e| GqlEntry { entry: e.clone() })
    }

    /// Check if fiber has a specific category
    async fn has_category(&self, category: GqlCategoryType) -> bool {
        self.inner.has_category(category.into())
    }

    // === Direct category accessors (no nulls) ===

    /// Get the index value directly (if present)
    async fn index_value(&self) -> Option<i32> {
        self.inner.get(CategoryType::Index)
            .and_then(|e| e.as_index())
            .map(|i| i.value() as i32)
    }

    /// Get node info directly (if present)
    async fn node(&self) -> Option<NodeInfo> {
        self.inner.get(CategoryType::Topological)
            .and_then(|e| e.as_node())
            .map(|n| NodeInfo {
                order: n.order(),
                position: n.position(),
            })
    }

    /// Get geometric info directly (if present)
    async fn geometric(&self) -> Option<GeometricInfo> {
        self.inner.get(CategoryType::Geometric)
            .and_then(|e| e.as_geometric())
            .map(|g| GeometricInfo {
                order: g.point.order(),
                position: g.point.position(),
                x: g.coordinates.x,
                y: g.coordinates.y,
                z: g.coordinates.z,
            })
    }

    /// Get lexicon info directly (if present)
    async fn lexicon(&self) -> Option<LexiconInfo> {
        self.inner.get(CategoryType::Lexicon)
            .and_then(|e| e.as_lexicon())
            .map(|l| LexiconInfo {
                vocabulary: "canonical".to_string(),
                term: l.term.clone(),
                description: l.description.clone(),
            })
    }

    /// Get color info directly (always available via position)
    async fn color(&self) -> ColorInfo {
        ColorInfo {
            color: Color::from_index(self.inner.position),
        }
    }

    /// The color hex code for this fiber position
    async fn hex_color(&self) -> String {
        Color::from_index(self.inner.position).canonical_hex().to_string()
    }
}

// ============================================================================
// System View
// ============================================================================

/// A complete generated system view
#[derive(Clone, Debug)]
pub struct GqlSystemView {
    inner: CoreSystemView,
}

impl GqlSystemView {
    pub fn new(inner: CoreSystemView) -> Self {
        Self { inner }
    }
}

#[Object]
impl GqlSystemView {
    /// The order of this system
    async fn order(&self) -> i32 {
        self.inner.order.value() as i32
    }

    /// The simplex scaffold
    async fn simplex(&self) -> GqlSimplexView {
        GqlSimplexView {
            inner: self.inner.simplex.clone(),
        }
    }

    /// All fibers in this system
    async fn fibers(&self) -> Vec<GqlFiberView> {
        self.inner.fibers.iter()
            .map(|f| GqlFiberView { inner: f.clone() })
            .collect()
    }

    /// Get fiber at a specific position (1-indexed)
    async fn fiber_at(&self, position: i32) -> Option<GqlFiberView> {
        Index::from_value(position as u8)
            .and_then(|idx| self.inner.fiber_at(idx))
            .map(|f| GqlFiberView { inner: f.clone() })
    }

    /// System name (from metadata)
    async fn name(&self) -> Option<String> {
        self.inner.name().map(|s| s.to_string())
    }

    /// Coherence attribute (from metadata)
    async fn coherence(&self) -> Option<String> {
        self.inner.coherence().map(|s| s.to_string())
    }

    /// Term designation (from metadata)
    async fn term_designation(&self) -> Option<String> {
        self.inner.get_metadata(MetadataType::TermDesignation).map(|s| s.to_string())
    }

    /// Connective designation (from metadata)
    async fn connective_designation(&self) -> Option<String> {
        self.inner.get_metadata(MetadataType::ConnectiveDesignation).map(|s| s.to_string())
    }

    /// Get specific metadata
    async fn metadata(&self, key: GqlMetadataType) -> Option<String> {
        self.inner.get_metadata(key.into()).map(|s| s.to_string())
    }

    /// The color for this system order
    async fn color(&self) -> ColorInfo {
        ColorInfo {
            color: Color::from_index(self.inner.order),
        }
    }

    /// The hex color code for this system
    async fn hex_color(&self) -> String {
        Color::from_index(self.inner.order).canonical_hex().to_string()
    }

    /// All connectives (1-simplices in vocabulary category)
    async fn connectives(&self) -> Vec<ConnectiveInfo> {
        to_connective_characters(self.inner.order)
            .into_iter()
            .map(|c| ConnectiveInfo {
                vocabulary: "canonical".to_string(),
                name: c.name.to_string(),
                from: c.from_term.to_string(),
                to: c.to_term.to_string(),
            })
            .collect()
    }
}

