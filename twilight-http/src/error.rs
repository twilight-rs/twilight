use crate::{api_error::ApiError, json::JsonError, response::StatusCode};
use http::Response;
use hyper::body::Incoming;
use std::{
    error::Error as StdError,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    str,
};

#[derive(Debug)]
pub struct Error {
    pub(super) source: Option<Box<dyn StdError + Send + Sync>>,
    pub(super) kind: ErrorType,
}

impl Error {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &ErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn StdError + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (ErrorType, Option<Box<dyn StdError + Send + Sync>>) {
        (self.kind, self.source)
    }

    pub(super) fn json(source: JsonError) -> Self {
        Self {
            kind: ErrorType::Json,
            source: Some(Box::new(source)),
        }
    }

    pub(super) fn validation(source: impl StdError + Send + Sync + 'static) -> Self {
        Self {
            kind: ErrorType::Validation,
            source: Some(Box::new(source)),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            ErrorType::BuildingRequest => f.write_str("failed to build the request"),
            ErrorType::ChunkingResponse => f.write_str("Chunking the response failed"),
            ErrorType::CreatingHeader { name, .. } => {
                f.write_str("Parsing the value for header {}")?;
                f.write_str(name)?;

                f.write_str(" failed")
            }
            ErrorType::Json => f.write_str("Given value couldn't be serialized"),
            ErrorType::Parsing { body, .. } => {
                f.write_str("Response body couldn't be deserialized: ")?;

                if let Ok(body) = str::from_utf8(body) {
                    f.write_str(body)
                } else {
                    Debug::fmt(body, f)
                }
            }
            ErrorType::RatelimiterTicket => f.write_str("Failed to get ratelimiter ticket"),
            ErrorType::RequestCanceled => {
                f.write_str("Request was canceled either before or while being sent")
            }
            ErrorType::RequestError => f.write_str("Parsing or sending the response failed"),
            ErrorType::RequestTimedOut => f.write_str("request timed out"),
            ErrorType::Response { body, status, .. } => {
                f.write_str("Response error: status code ")?;
                Display::fmt(status, f)?;
                f.write_str(", error: ")?;

                f.write_str(&String::from_utf8_lossy(body))
            }
            ErrorType::ServiceUnavailable { .. } => {
                f.write_str("api may be temporarily unavailable (received a 503)")
            }
            ErrorType::Unauthorized => {
                f.write_str("token in use is invalid, expired, or is revoked")
            }
            ErrorType::Validation => f.write_str("request fields have invalid values"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn StdError + 'static))
    }
}

/// Type of [`Error`] that occurred.
#[non_exhaustive]
pub enum ErrorType {
    BuildingRequest,
    ChunkingResponse,
    CreatingHeader {
        name: String,
    },
    Json,
    Parsing {
        body: Vec<u8>,
    },
    RatelimiterTicket,
    RequestCanceled,
    RequestError,
    RequestTimedOut,
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
        response: Response<Incoming>,
    },
    /// Token in use has become revoked or is otherwise invalid.
    ///
    /// This can occur if a bot token is invalidated or an access token expires
    /// or is revoked. Recreate the client to configure a new token.
    Unauthorized,
    /// A field failed validation requirements during request building.
    ///
    /// The inputs of request methods for fields are validated for correctness.
    /// For example, [`CreateMessage::content`] is validated to ensure that the
    /// message content isn't too long; [`ExecuteWebhook::embeds`] is validated
    /// to ensure that a correct number of embeds are provided; and so on.
    ///
    /// Validation failures aren't immediately returned; rather, validation
    /// errors are returned when calling the [`IntoFuture`] or
    /// [`TryIntoRequest`] implementations on requests.
    ///
    /// # Examples
    ///
    /// Passing a message with valid content succeeds as expected:
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let channel_id = twilight_model::id::Id::new(1);
    /// use std::env;
    /// use twilight_http::{client::Client, request::TryIntoRequest};
    ///
    /// let client = Client::new(env::var("DISCORD_TOKEN")?);
    /// let builder = client.create_message(channel_id).content("Ping!");
    ///
    /// assert!(builder.try_into_request().is_ok());
    /// # Ok(()) }
    /// ```
    ///
    /// However, passing an invalid content returns a validation error upon
    /// finalizing the request building:
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let channel_id = twilight_model::id::Id::new(1);
    /// use std::{env, error::Error};
    /// use twilight_http::{client::Client, error::ErrorType, request::TryIntoRequest};
    ///
    /// let client = Client::new(env::var("DISCORD_TOKEN")?);
    ///
    /// // this is a very long message
    /// let content = "pinkie pie is cool ".repeat(1000);
    ///
    /// let builder = client.create_message(channel_id).content(&content);
    ///
    /// let error = builder.try_into_request().unwrap_err();
    /// assert!(matches!(error.kind(), ErrorType::Validation));
    ///
    /// // print the contents of the validation error
    /// println!("{:?}", error.source());
    /// # Ok(()) }
    /// ```
    ///
    /// [`CreateMessage::content`]: crate::request::channel::message::create_message::CreateMessage
    /// [`ExecuteWebhook::embeds`]: crate::request::channel::webhook::execute_webhook::ExecuteWebhook
    /// [`IntoFuture`]: std::future::IntoFuture
    /// [`TryIntoRequest`]: crate::request::TryIntoRequest
    Validation,
}

impl Debug for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::BuildingRequest => f.write_str("BuildingRequest"),
            Self::ChunkingResponse => f.write_str("ChunkingResponse"),
            Self::CreatingHeader { name } => f
                .debug_struct("CreatingHeader")
                .field("name", name)
                .finish(),
            Self::Json => f.write_str("Json"),
            Self::Parsing { body } => {
                let mut debug = f.debug_struct("Parsing");

                if let Ok(body_string) = str::from_utf8(body) {
                    debug.field("body_string", &body_string);
                }

                debug.field("body", body).finish()
            }
            Self::RatelimiterTicket => f.write_str("RatelimiterTicket"),
            Self::RequestCanceled => f.write_str("RequestCanceled"),
            Self::RequestError => f.write_str("RequestError"),
            Self::RequestTimedOut => f.write_str("RequestTimedOut"),
            Self::Response {
                body,
                error,
                status,
            } => {
                let mut debug = f.debug_struct("Response");

                if let Ok(body_string) = str::from_utf8(body) {
                    debug.field("body_string", &body_string);
                }

                debug
                    .field("body", body)
                    .field("error", error)
                    .field("status", status)
                    .finish()
            }
            Self::ServiceUnavailable { response } => f
                .debug_struct("ServiceUnavailable")
                .field("response", response)
                .finish(),
            Self::Unauthorized => f.write_str("Unauthorized"),
            Self::Validation => f.write_str("Validation"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ErrorType;
    use crate::{
        api_error::{ApiError, GeneralApiError},
        response::StatusCode,
    };

    /// Ensure
    #[test]
    fn parsing_variant_debug() {
        let body = br#"{"message": "aaa"#.to_vec();

        let error = ErrorType::Parsing { body };

        assert_eq!(
            "Parsing {
    body_string: \"{\\\"message\\\": \\\"aaa\",
    body: [
        123,
        34,
        109,
        101,
        115,
        115,
        97,
        103,
        101,
        34,
        58,
        32,
        34,
        97,
        97,
        97,
    ],
}",
            format!("{error:#?}"),
        );
    }

    #[test]
    fn response_variant_debug() {
        let body = br#"{"message": "aaa"}"#.to_vec();

        let error = ErrorType::Response {
            body,
            error: ApiError::General(GeneralApiError {
                code: 0,
                message: "401: Unauthorized".to_owned(),
            }),
            status: StatusCode::new(401),
        };

        assert_eq!(
            "Response {
    body_string: \"{\\\"message\\\": \\\"aaa\\\"}\",
    body: [
        123,
        34,
        109,
        101,
        115,
        115,
        97,
        103,
        101,
        34,
        58,
        32,
        34,
        97,
        97,
        97,
        34,
        125,
    ],
    error: General(
        GeneralApiError {
            code: 0,
            message: \"401: Unauthorized\",
        },
    ),
    status: StatusCode(
        401,
    ),
}",
            format!("{error:#?}"),
        );
    }
}
