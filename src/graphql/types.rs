use async_graphql::*;
use crate::core::{Index, Color};
use crate::core::generators::{generate_simplex, generate_fiber, generate_fiber_with, generate_system, generate_system_with};
use crate::core::config::{FiberConfig, SystemConfig};
use crate::core::transforms::{from_system_name, from_coherence, from_edge_count};
use crate::graphql::category_types::*;

/// Root query object
#[derive(Clone, Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // ========================================================================
    // Category-Theoretic API (Generator-Based Queries)
    // ========================================================================

    // === Index Queries ===

    /// Get information about an index value (1-12)
    async fn index(&self, value: i32) -> Option<IndexInfo> {
        Index::from_value(value as u8).map(|index| IndexInfo::new(index))
    }

    /// Get all valid index values with their metadata
    async fn all_indices(&self) -> Vec<IndexInfo> {
        (1..=12)
            .filter_map(Index::from_value)
            .map(|index| IndexInfo::new(index))
            .collect()
    }

    /// Look up index by system name (co-determination)
    async fn index_by_name(&self, name: String) -> Option<IndexInfo> {
        from_system_name(&name).map(|index| IndexInfo::new(index))
    }

    /// Look up index by coherence attribute (co-determination)
    async fn index_by_coherence(&self, coherence: String) -> Option<IndexInfo> {
        from_coherence(&coherence).map(|index| IndexInfo::new(index))
    }

    /// Look up index by edge count (co-determination)
    async fn index_by_edge_count(&self, count: i32) -> Option<IndexInfo> {
        from_edge_count(count as u8).map(|index| IndexInfo::new(index))
    }

    // === Color Queries ===

    /// Get information about a color value (1-12)
    async fn color(&self, value: i32) -> Option<ColorInfo> {
        if value < 1 || value > 12 {
            return None;
        }
        Index::from_value(value as u8).map(|idx| ColorInfo::new(Color::from_index(idx)))
    }

    /// Get all colors with their metadata
    async fn all_colors(&self) -> Vec<ColorInfo> {
        (1..=12)
            .filter_map(Index::from_value)
            .map(|idx| ColorInfo::new(Color::from_index(idx)))
            .collect()
    }

    // === Simplex Queries ===

    /// Generate a simplex scaffold (Kn complete graph) for a given order
    async fn simplex(&self, order: i32) -> Option<GqlSimplexView> {
        Index::from_value(order as u8).map(|idx| GqlSimplexView::new(generate_simplex(idx)))
    }

    // === Fiber Queries ===

    /// Generate a fiber with default categories (Index, Geometric, Vocabulary)
    async fn fiber(&self, order: i32, position: i32) -> Option<GqlFiberView> {
        let order_idx = Index::from_value(order as u8)?;
        let pos_idx = Index::from_value(position as u8)?;
        generate_fiber(order_idx, pos_idx).map(|f| GqlFiberView::new(f))
    }

    /// Generate a fiber with custom configuration
    async fn fiber_with(
        &self,
        order: i32,
        position: i32,
        config: FiberConfigInput,
    ) -> Option<GqlFiberView> {
        let order_idx = Index::from_value(order as u8)?;
        let pos_idx = Index::from_value(position as u8)?;

        let fiber_config = FiberConfig {
            categories: config.categories
                .map(|cats| cats.into_iter().map(|c| c.into()).collect())
                .unwrap_or_else(|| FiberConfig::default().categories),
        };

        generate_fiber_with(order_idx, pos_idx, &fiber_config).map(|f| GqlFiberView::new(f))
    }

    // === Generated System Queries ===

    /// Generate a system view with default configuration
    async fn generate_system(&self, order: i32) -> Option<GqlSystemView> {
        Index::from_value(order as u8).map(|idx| GqlSystemView::new(generate_system(idx)))
    }

    /// Generate a system view with custom configuration
    async fn generate_system_with(
        &self,
        order: i32,
        config: SystemConfigInput,
    ) -> Option<GqlSystemView> {
        let order_idx = Index::from_value(order as u8)?;

        let system_config = SystemConfig {
            categories: config.categories
                .map(|cats| cats.into_iter().map(|c| c.into()).collect())
                .unwrap_or_else(|| SystemConfig::default().categories),
            include_metadata: config.include_metadata
                .map(|metas| metas.into_iter().map(|m| m.into()).collect())
                .unwrap_or_else(|| SystemConfig::default().include_metadata),
        };

        Some(GqlSystemView::new(generate_system_with(order_idx, &system_config)))
    }

    /// Generate all systems (1-12) with default configuration
    async fn generate_all_systems(&self) -> Vec<GqlSystemView> {
        (1..=12)
            .filter_map(Index::from_value)
            .map(|idx| GqlSystemView::new(generate_system(idx)))
            .collect()
    }

    /// Generate a system with all categories and all metadata
    async fn generate_full_system(&self, order: i32) -> Option<GqlSystemView> {
        let order_idx = Index::from_value(order as u8)?;
        Some(GqlSystemView::new(generate_system_with(order_idx, &SystemConfig::full())))
    }

    // === Category Info ===

    /// Get all available category types
    async fn category_types(&self) -> Vec<GqlCategoryType> {
        vec![
            GqlCategoryType::Index,
            GqlCategoryType::Topological,
            GqlCategoryType::Geometric,
            GqlCategoryType::Lexicon,
            GqlCategoryType::Color,
        ]
    }

    /// Get all available metadata types
    async fn metadata_types(&self) -> Vec<GqlMetadataType> {
        vec![
            GqlMetadataType::Name,
            GqlMetadataType::Coherence,
            GqlMetadataType::TermDesignation,
            GqlMetadataType::ConnectiveDesignation,
        ]
    }
}

pub type SystematicsSchema = async_graphql::Schema<QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

pub fn create_schema() -> SystematicsSchema {
    async_graphql::Schema::build(QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription)
        .finish()
}
