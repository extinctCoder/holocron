use indexmap::IndexMap;
use serde::Deserialize;

/// A `CREATE TABLE`: physical storage. Columns are an ordered map so emitted DDL
/// is deterministic (insertion order is preserved).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Table {
    pub name: String,
    #[serde(default)]
    pub if_not_exists: bool,
    pub columns: IndexMap<String, Column>,
    #[serde(default)]
    pub primary_key: Option<PrimaryKey>,
    #[serde(default)]
    pub indexes: Vec<Index>,
}

/// One column definition: a type, plus nullability and an optional default.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Column {
    /// Built-in type name or a declared enum name; resolved in a later phase.
    pub r#type: String,
    /// `true` permits NULL; columns are NOT NULL by default.
    #[serde(default)]
    pub null: bool,
    /// Raw SQL default expression, passed through verbatim.
    #[serde(default)]
    pub default: Option<String>,
}

/// The column(s) that uniquely identify a row.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrimaryKey {
    #[serde(default)]
    pub name: Option<String>,
    pub columns: Vec<String>,
}

/// A `CREATE INDEX`: a lookup shortcut; affects performance, never results.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Index {
    pub name: String,
    pub columns: Vec<String>,
    #[serde(default)]
    pub unique: bool,
    /// Index method (`btree`, `gin`, …); the engine's default when absent.
    #[serde(default)]
    pub using: Option<String>,
    /// Partial-index predicate, raw SQL, passed through verbatim.
    #[serde(default)]
    pub r#where: Option<String>,
}
