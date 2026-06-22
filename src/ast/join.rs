use serde::Deserialize;

/// A view's `from:` — the source relation and the alias the rest of the view uses.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FromClause {
    pub table: String,
    pub r#as: String,
}

/// A `JOIN … ON`: combine rows from another relation by a matching rule.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Join {
    pub table: String,
    pub r#as: String,
    /// Join kind; a bare join is `INNER`, matching SQL.
    #[serde(default)]
    pub r#type: JoinKind,
    /// The match condition, raw SQL, passed through verbatim (escape hatch).
    pub on: String,
}

/// Which kind of join. `INNER` keeps only matches; `LEFT` keeps all left rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum JoinKind {
    #[default]
    Inner,
    Left,
    Right,
    Full,
}
