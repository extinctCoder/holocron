use serde::Deserialize;

/// A `CREATE TYPE … AS ENUM`: a named, fixed set of allowed string values.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnumType {
    pub name: String,
    pub r#enum: Vec<String>,
}
