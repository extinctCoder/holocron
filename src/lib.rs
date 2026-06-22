//! Declarative schema & query compiler — one YAML as the source of truth.

mod ast;
mod catalog;
mod emit;
mod error;
mod query;
mod resolve;

pub use ast::{
    parse_schema, Column, EnumType, FromClause, Index, Join, JoinKind, PrimaryKey, SchemaDocument,
    SelectColumn, SelectExpression, SelectItem, Table, View,
};
pub use catalog::{
    build_catalog, Catalog, CatalogColumn, CatalogRelation, CatalogType, RelationKind,
};
pub use emit::emit_schema;
pub use error::HolocronError;
pub use query::{check_query, CheckedQuery, CompareOp, Comparison, Filter, Query, SetOp};
pub use resolve::resolve_views;
