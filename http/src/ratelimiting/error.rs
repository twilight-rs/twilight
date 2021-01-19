use hyper::header::ToStrError;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    result::Result as StdResult,
};

pub type RatelimitResult<T> = StdResult<T, RatelimitError>;

#[derive(Debug)]
pub struct RatelimitError {
    pub(super) source: Option<Box<dyn Error + Send + Sync>>,
    pub(super) kind: RatelimitErrorType,
}

impl RatelimitError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub fn kind(&self) -> &RatelimitErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (RatelimitErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }

    pub(super) fn header_missing(name: &'static str) -> Self {
        Self {
            kind: RatelimitErrorType::HeaderMissing { name },
            source: None,
        }
    }

    pub(super) fn header_not_utf8(name: &'static str, value: Vec<u8>, source: ToStrError) -> Self {
        Self {
            kind: RatelimitErrorType::HeaderNotUtf8 { name, value },
            source: Some(Box::new(source)),
        }
    }
}

impl Display for RatelimitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            RatelimitErrorType::NoHeaders => f.write_str("No headers are present"),
            RatelimitErrorType::HeaderMissing { name } => {
                write!(f, "At least one header, {:?}, is missing", name)
            }
            RatelimitErrorType::HeaderNotUtf8 { name, value, .. } => {
                write!(f, "The header {:?} has invalid UTF-16: {:?}", name, value)
            }
            RatelimitErrorType::ParsingBoolText { name, text, .. } => write!(
                f,
                "The header {:?} should be a bool but isn't: {:?}",
                name, text
            ),
            RatelimitErrorType::ParsingFloatText { name, text, .. } => write!(
                f,
                "The header {:?} should be a float but isn't: {:?}",
                name, text
            ),
            RatelimitErrorType::ParsingIntText { name, text, .. } => write!(
                f,
                "The header {:?} should be an integer but isn't: {:?}",
                name, text
            ),
        }
    }
}

impl Error for RatelimitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum RatelimitErrorType {
    NoHeaders,
    HeaderMissing { name: &'static str },
    HeaderNotUtf8 { name: &'static str, value: Vec<u8> },
    ParsingBoolText { name: &'static str, text: String },
    ParsingFloatText { name: &'static str, text: String },
    ParsingIntText { name: &'static str, text: String },
}
