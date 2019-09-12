use crate::ratelimiting::RatelimitError;
use futures_channel::oneshot::Canceled;
use http::{
    header::InvalidHeaderValue,
    method::Method,
    status::StatusCode,
    Error as HttpError,
};
use reqwest::{Error as ReqwestError, Response as ReqwestResponse};
use serde_json::Error as JsonError;
use snafu::Snafu;
use std::{
    fmt::Error as FmtError,
    result::Result as StdResult,
};
use url::ParseError;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ResponseError {
    /// A 4xx response status code. Submit a GitHub issue with this error so we
    /// can fix it.
    Client {
        response: ReqwestResponse,
    },
    /// A 5xx response status code. These are internal errors on Discord's side.
    Server {
        response: ReqwestResponse,
    },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
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
    InvalidUrl {
        method: Method,
        path: String,
        source: ParseError,
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
    RequestBuilding {
        path: String,
        source: HttpError,
    },
    RequestCanceled {
        source: Canceled,
    },
    RequestError {
        method: Method,
        source: ReqwestError,
    },
    Response {
        source: ResponseError,
    },
    StreamingPayload {
        status: StatusCode,
        source: ReqwestError,
    },
}

impl From<FmtError> for Error {
    fn from(e: FmtError) -> Self {
        Error::Formatting {
            source: e,
        }
    }
}

impl From<JsonError> for Error {
    fn from(e: JsonError) -> Self {
        Error::Json {
            source: e,
        }
    }
}
