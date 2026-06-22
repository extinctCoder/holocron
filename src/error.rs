use thiserror::Error;

/// The crate's single root error type. Each layer wraps the one below it.
#[derive(Debug, Error)]
pub enum HolocronError {
    /// The YAML did not fit the declared schema shape (Layer 1).
    #[error("parse error: {0}")]
    Parse(String),
}

impl HolocronError {
    /// Build a [`HolocronError::Parse`]; callers pass a message, never the variant.
    pub(crate) fn parse(message: impl Into<String>) -> Self {
        Self::Parse(message.into())
    }
}
