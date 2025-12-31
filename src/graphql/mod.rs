pub mod types;
pub mod category_types;

pub use types::{create_schema, SystematicsSchema, QueryRoot};
pub use category_types::{
    GqlCategoryType, GqlMetadataType,
    FiberConfigInput, SystemConfigInput,
    IndexInfo, ColorInfo,
    GqlEntry, NodeInfo, GeometricInfo, LexiconInfo, ConnectiveInfo,
    GqlSimplexView, EdgePair,
    GqlFiberView, GqlSystemView,
};
