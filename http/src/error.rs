use crate::{api_error::ApiError, ratelimiting::RatelimitError};
use futures_channel::oneshot::Canceled;
use reqwest::{header::InvalidHeaderValue, Error as ReqwestError, Response, StatusCode};
use std::{
    error::Error as StdError,
    fmt::{Display, Error as FmtError, Formatter, Result as FmtResult},
    num::ParseIntError,
    result::Result as StdResult,
};
use url::ParseError as UrlParseError;

#[cfg(not(feature = "simd-json"))]
use serde_json::Error as JsonError;
#[cfg(feature = "simd-json")]
use simd_json::Error as JsonError;

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug)]
#[non_exhaustive]
pub enum UrlError {
    UrlParsing { source: UrlParseError },
    IdParsing { source: ParseIntError },
    SegmentMissing,
}

impl Display for UrlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::UrlParsing { source, .. } => write!(f, "Url path couldn't be parsed: {}", source),
            Self::IdParsing { source, .. } => {
                write!(f, "Url path segment wasn't a valid ID: {}", source)
            }
            Self::SegmentMissing => f.write_str("Url was missing a required path segment"),
        }
    }
}

impl StdError for UrlError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::UrlParsing { source, .. } => Some(source),
            Self::IdParsing { source, .. } => Some(source),
            Self::SegmentMissing => None,
        }
    }
}

impl From<UrlParseError> for UrlError {
    fn from(source: UrlParseError) -> Self {
        Self::UrlParsing { source }
    }
}

impl From<ParseIntError> for UrlError {
    fn from(source: ParseIntError) -> Self {
        Self::IdParsing { source }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    BuildingClient {
        source: ReqwestError,
    },
    ChunkingResponse {
        source: ReqwestError,
    },
    CreatingHeader {
        name: String,
        source: InvalidHeaderValue,
    },
    Formatting {
        source: FmtError,
    },
    Json {
        source: JsonError,
    },
    Parsing {
        body: Vec<u8>,
        source: JsonError,
    },
    Url {
        source: UrlError,
    },
    Ratelimiting {
        source: RatelimitError,
    },
    RequestCanceled {
        source: Canceled,
    },
    RequestError {
        source: ReqwestError,
    },
    Response {
        body: Vec<u8>,
        error: ApiError,
        status: StatusCode,
    },
    /// API service is unavailable. Consider re-sending the request at a
    /// later time.
    ///
    /// This may occur during Discord API stability incidents.
    ServiceUnavailable {
        response: Response,
    },
    /// Token in use has become revoked or is otherwise invalid.
    ///
    /// This can occur if a bot token is invalidated or an access token expires
    /// or is revoked. Recreate the client to configure a new token.
    Unauthorized,
}

impl From<FmtError> for Error {
    fn from(source: FmtError) -> Self {
        Self::Formatting { source }
    }
}

impl From<JsonError> for Error {
    fn from(source: JsonError) -> Self {
        Self::Json { source }
    }
}

impl From<UrlError> for Error {
    fn from(source: UrlError) -> Self {
        Self::Url { source }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::BuildingClient { .. } => {
                f.write_str("HTTP client couldn't be built due to a reqwest client error")
            }
            Self::ChunkingResponse { .. } => f.write_str("Chunking the response failed"),
            Self::CreatingHeader { name, .. } => {
                write!(f, "Parsing the value for header {} failed", name)
            }
            Self::Formatting { .. } => f.write_str("Formatting a string failed"),
            Self::Json { .. } => f.write_str("Given value couldn't be serialized"),
            Self::Parsing { body, .. } => {
                write!(f, "Response body couldn't be deserialized: {:?}", body)
            }
            Self::Url { source, .. } => write!(f, "{}", source),
            Self::Ratelimiting { .. } => f.write_str("Ratelimiting failure"),
            Self::RequestCanceled { .. } => {
                f.write_str("Request was canceled either before or while being sent")
            }
            Self::RequestError { .. } => f.write_str("Parsing or sending the response failed"),
            Self::Response { error, status, .. } => write!(
                f,
                "Response error: status code {}, error: {}",
                status, error
            ),
            Self::ServiceUnavailable { .. } => {
                f.write_str("api may be temporarily unavailable (received a 503)")
            }
            Self::Unauthorized => f.write_str("token in use is invalid, expired, or is revoked"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::CreatingHeader { source, .. } => Some(source),
            Self::Formatting { source } => Some(source),
            Self::Json { source } | Self::Parsing { source, .. } => Some(source),
            Self::Url { source } => Some(source),
            Self::Ratelimiting { source } => Some(source),
            Self::RequestCanceled { source } => Some(source),
            Self::BuildingClient { source }
            | Self::ChunkingResponse { source }
            | Self::RequestError { source } => Some(source),
            Self::Response { .. } | Self::ServiceUnavailable { .. } | Self::Unauthorized => None,
        }
    }
}
