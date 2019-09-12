use http::header::ToStrError;
use snafu::Snafu;
use std::{
    num::{ParseFloatError, ParseIntError},
    result::Result as StdResult,
    str::ParseBoolError,
};

pub type RatelimitResult<T> = StdResult<T, RatelimitError>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
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
