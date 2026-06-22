use crate::catalog::data_type::CatalogType;

/// A resolved column in the catalog: its name, type, nullability, and the
/// per-column query policy resolved from view select-item flags (or the
/// defaults applied to table columns).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogColumn {
    pub name: String,
    pub data_type: CatalogType,
    /// `true` if the column permits NULL.
    pub nullable: bool,
    /// `true` if queries may filter on this column.
    pub filterable: bool,
    /// `true` if free-text search includes this column.
    pub searchable: bool,
}
