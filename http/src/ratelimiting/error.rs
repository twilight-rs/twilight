use hyper::header::ToStrError;
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    num::{ParseFloatError, ParseIntError},
    result::Result as StdResult,
    str::ParseBoolError,
};

pub type RatelimitResult<T> = StdResult<T, RatelimitError>;

#[derive(Debug)]
#[non_exhaustive]
pub enum RatelimitError {
    NoHeaders,
    HeaderMissing {
        name: &'static str,
    },
    HeaderNotUtf8 {
        name: &'static str,
        source: ToStrError,
        value: Vec<u8>,
    },
    ParsingBoolText {
        name: &'static str,
        source: ParseBoolError,
        text: String,
    },
    ParsingFloatText {
        name: &'static str,
        source: ParseFloatError,
        text: String,
    },
    ParsingIntText {
        name: &'static str,
        source: ParseIntError,
        text: String,
    },
}

impl Display for RatelimitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NoHeaders => f.write_str("No headers are present"),
            Self::HeaderMissing { name } => {
                write!(f, "At least one header, {:?}, is missing", name)
            }
            Self::HeaderNotUtf8 { name, value, .. } => {
                write!(f, "The header {:?} has invalid UTF-16: {:?}", name, value)
            }
            Self::ParsingBoolText { name, text, .. } => write!(
                f,
                "The header {:?} should be a bool but isn't: {:?}",
                name, text
            ),
            Self::ParsingFloatText { name, text, .. } => write!(
                f,
                "The header {:?} should be a float but isn't: {:?}",
                name, text
            ),
            Self::ParsingIntText { name, text, .. } => write!(
                f,
                "The header {:?} should be an integer but isn't: {:?}",
                name, text
            ),
        }
    }
}

impl StdError for RatelimitError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::HeaderNotUtf8 { source, .. } => Some(source),
            Self::ParsingBoolText { source, .. } => Some(source),
            Self::ParsingFloatText { source, .. } => Some(source),
            Self::ParsingIntText { source, .. } => Some(source),
            Self::NoHeaders | Self::HeaderMissing { .. } => None,
        }
    }
}
