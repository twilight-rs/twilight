use crate::{api_error::ApiError, ratelimiting::RatelimitError};
use futures_channel::oneshot::Canceled;
use hyper::{
    header::InvalidHeaderValue, http::Error as HttpError, Body, Error as HyperError, Response,
    StatusCode,
};
use std::{
    error::Error as StdError,
    fmt::{Display, Error as FmtError, Formatter, Result as FmtResult},
    result::Result as StdResult,
};
use tokio::time::error::Elapsed;

#[cfg(not(feature = "simd-json"))]
use serde_json::Error as JsonError;
#[cfg(feature = "simd-json")]
use simd_json::Error as JsonError;

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    BuildingRequest {
        source: HttpError,
    },
    ChunkingResponse {
        source: HyperError,
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
    Ratelimiting {
        source: RatelimitError,
    },
    RequestCanceled {
        source: Canceled,
    },
    RequestError {
        source: HyperError,
    },
    RequestTimedOut {
        /// Source of the error when the request timed out.
        source: Elapsed,
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
        response: Response<Body>,
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

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::BuildingRequest { .. } => f.write_str("failed to build the request"),
            Self::ChunkingResponse { .. } => f.write_str("Chunking the response failed"),
            Self::CreatingHeader { name, .. } => {
                write!(f, "Parsing the value for header {} failed", name)
            }
            Self::Formatting { .. } => f.write_str("Formatting a string failed"),
            Self::Json { .. } => f.write_str("Given value couldn't be serialized"),
            Self::Parsing { body, .. } => {
                write!(f, "Response body couldn't be deserialized: {:?}", body)
            }
            Self::Ratelimiting { .. } => f.write_str("Ratelimiting failure"),
            Self::RequestCanceled { .. } => {
                f.write_str("Request was canceled either before or while being sent")
            }
            Self::RequestError { .. } => f.write_str("Parsing or sending the response failed"),
            Self::RequestTimedOut { .. } => f.write_str("request timed out"),
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
            Self::BuildingRequest { source } => Some(source),
            Self::CreatingHeader { source, .. } => Some(source),
            Self::Formatting { source } => Some(source),
            Self::Json { source } | Self::Parsing { source, .. } => Some(source),
            Self::Ratelimiting { source } => Some(source),
            Self::RequestCanceled { source } => Some(source),
            Self::ChunkingResponse { source } | Self::RequestError { source } => Some(source),
            Self::RequestTimedOut { source } => Some(source),
            Self::Response { .. } | Self::ServiceUnavailable { .. } | Self::Unauthorized => None,
        }
    }
}
