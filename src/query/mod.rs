//! The query layer: a filter expression checked against the catalog.

mod check;
mod filter;

pub use check::{check_query, CheckedQuery};
pub use filter::{CompareOp, Comparison, Filter, SetOp};

/// A query against a relation: a filter expression to apply.
///
/// Surface syntaxes (RSQL, YAML) parse into this shared model before checking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Query {
    /// The relation (table or view) the query targets.
    pub relation: String,
    /// The filter expression; `None` matches every row.
    pub filter: Option<Filter>,
}
