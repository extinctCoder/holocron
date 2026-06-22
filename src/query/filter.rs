use crate::catalog::CatalogType;

/// A filter expression tree: logical AND/OR combinators around comparison leaves.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Filter {
    And(Vec<Filter>),
    Or(Vec<Filter>),
    Leaf(Comparison),
}

/// A single comparison against a column. The shape of the value is fixed by
/// the variant — illegal states (e.g. `In` with one value) are unrepresentable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Comparison {
    /// A column-to-value comparison: equality, ordering, or text pattern.
    Compare {
        column: String,
        op: CompareOp,
        value: String,
    },
    /// Membership in a set of values.
    Set {
        column: String,
        op: SetOp,
        values: Vec<String>,
    },
    /// A null check: `true` for IS NULL, `false` for IS NOT NULL.
    NullCheck { column: String, is_null: bool },
}

/// Column-to-value comparison operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOp {
    Eq,
    NotEq,
    Gt,
    Gte,
    Lt,
    Lte,
    Like,
    StartsWith,
    EndsWith,
}

/// Set-membership operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetOp {
    In,
    NotIn,
}

impl CompareOp {
    /// Whether this operator is valid for the given column type.
    pub fn supported_by(&self, data_type: &CatalogType) -> bool {
        match self {
            Self::Eq | Self::NotEq => data_type.supports_equality(),
            Self::Gt | Self::Gte | Self::Lt | Self::Lte => data_type.supports_ordering(),
            Self::Like | Self::StartsWith | Self::EndsWith => data_type.supports_text_pattern(),
        }
    }

    /// Surface-syntax name, used in diagnostics.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Eq => "==",
            Self::NotEq => "!=",
            Self::Gt => "=gt=",
            Self::Gte => "=ge=",
            Self::Lt => "=lt=",
            Self::Lte => "=le=",
            Self::Like => "=like=",
            Self::StartsWith => "=starts=",
            Self::EndsWith => "=ends=",
        }
    }
}

impl SetOp {
    /// Whether this operator is valid for the given column type.
    pub fn supported_by(&self, data_type: &CatalogType) -> bool {
        data_type.supports_set()
    }

    /// Surface-syntax name, used in diagnostics.
    pub fn name(&self) -> &'static str {
        match self {
            Self::In => "=in=",
            Self::NotIn => "=out=",
        }
    }
}
