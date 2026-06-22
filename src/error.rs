use thiserror::Error;

/// The crate's single root error type. Each layer wraps the one below it.
#[derive(Debug, Error)]
pub enum HolocronError {
    /// The YAML did not fit the declared schema shape (Layer 1).
    #[error("parse error: {0}")]
    Parse(String),

    /// A column's type is neither a built-in nor a declared enum.
    #[error("unknown type `{type_name}` on column `{relation}.{column}`")]
    UnknownType {
        relation: String,
        column: String,
        type_name: String,
    },

    /// Two relations share a name.
    #[error("duplicate relation `{0}`")]
    DuplicateRelation(String),

    /// Two enum types share a name.
    #[error("duplicate enum type `{0}`")]
    DuplicateEnum(String),

    /// A view source references a relation that does not exist.
    #[error("view `{view}`: source `{alias}` references unknown relation `{relation}`")]
    UnknownSource {
        view: String,
        alias: String,
        relation: String,
    },

    /// Two sources in a view share an alias.
    #[error("view `{view}`: duplicate alias `{alias}`")]
    DuplicateAlias { view: String, alias: String },

    /// A select item's `from` is not a declared alias in the view.
    #[error("view `{view}`: select references unknown alias `{alias}`")]
    UnknownAlias { view: String, alias: String },

    /// A select omitted `from` but the view has more than one source.
    #[error("view `{view}`: column `{column}` needs an explicit `from` (multiple sources)")]
    AmbiguousSource { view: String, column: String },

    /// A select references a column the relation does not have.
    #[error("column `{column}` does not exist in relation `{relation}`")]
    UnknownColumn { relation: String, column: String },

    /// A construct that is recognised but not yet implemented.
    #[error("unsupported: {0}")]
    Unsupported(String),

    /// A query targets a relation the catalog has no entry for.
    #[error("unknown relation `{0}`")]
    UnknownRelation(String),

    /// A query filter references a column declared non-filterable.
    #[error("column `{relation}.{column}` is not filterable")]
    NotFilterable { relation: String, column: String },

    /// An operator is not valid for the column's type.
    #[error("operator `{operator}` not supported on `{relation}.{column}` of type `{data_type}`")]
    OperatorNotSupported {
        relation: String,
        column: String,
        data_type: String,
        operator: String,
    },
}

impl HolocronError {
    pub(crate) fn parse(message: impl Into<String>) -> Self {
        Self::Parse(message.into())
    }

    pub(crate) fn unknown_type(
        relation: impl Into<String>,
        column: impl Into<String>,
        type_name: impl Into<String>,
    ) -> Self {
        Self::UnknownType {
            relation: relation.into(),
            column: column.into(),
            type_name: type_name.into(),
        }
    }

    pub(crate) fn duplicate_relation(name: impl Into<String>) -> Self {
        Self::DuplicateRelation(name.into())
    }

    pub(crate) fn duplicate_enum(name: impl Into<String>) -> Self {
        Self::DuplicateEnum(name.into())
    }

    pub(crate) fn unknown_source(
        view: impl Into<String>,
        alias: impl Into<String>,
        relation: impl Into<String>,
    ) -> Self {
        Self::UnknownSource {
            view: view.into(),
            alias: alias.into(),
            relation: relation.into(),
        }
    }

    pub(crate) fn duplicate_alias(view: impl Into<String>, alias: impl Into<String>) -> Self {
        Self::DuplicateAlias {
            view: view.into(),
            alias: alias.into(),
        }
    }

    pub(crate) fn unknown_alias(view: impl Into<String>, alias: impl Into<String>) -> Self {
        Self::UnknownAlias {
            view: view.into(),
            alias: alias.into(),
        }
    }

    pub(crate) fn ambiguous_source(view: impl Into<String>, column: impl Into<String>) -> Self {
        Self::AmbiguousSource {
            view: view.into(),
            column: column.into(),
        }
    }

    pub(crate) fn unknown_column(relation: impl Into<String>, column: impl Into<String>) -> Self {
        Self::UnknownColumn {
            relation: relation.into(),
            column: column.into(),
        }
    }

    pub(crate) fn unsupported(message: impl Into<String>) -> Self {
        Self::Unsupported(message.into())
    }

    pub(crate) fn unknown_relation(name: impl Into<String>) -> Self {
        Self::UnknownRelation(name.into())
    }

    pub(crate) fn not_filterable(relation: impl Into<String>, column: impl Into<String>) -> Self {
        Self::NotFilterable {
            relation: relation.into(),
            column: column.into(),
        }
    }

    pub(crate) fn operator_not_supported(
        relation: impl Into<String>,
        column: impl Into<String>,
        data_type: impl Into<String>,
        operator: impl Into<String>,
    ) -> Self {
        Self::OperatorNotSupported {
            relation: relation.into(),
            column: column.into(),
            data_type: data_type.into(),
            operator: operator.into(),
        }
    }
}
