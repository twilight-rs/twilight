use super::GetChannelMessagesConfigured;
use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::ListBody},
};
use std::future::IntoFuture;
use twilight_model::{
    channel::Message,
    id::{
        Id,
        marker::{ChannelMarker, MessageMarker},
    },
};
use twilight_validate::request::{
    ValidationError, get_channel_messages_limit as validate_get_channel_messages_limit,
};

pub(super) struct GetChannelMessagesPath {
    pub channel_id: Id<ChannelMarker>,
    pub limit: Option<u16>,
}

/// Get channel messages, by [`Id<ChannelMarker>`].
///
/// Only one of [`after`], [`around`], and [`before`] can be specified at a time.
/// Once these are specified, the type returned is [`GetChannelMessagesConfigured`].
///
/// If [`limit`] is unspecified, the default set by Discord is 50.
///
/// # Examples
///
/// ```no_run
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
/// let channel_id = Id::new(123);
/// let message_id = Id::new(234);
///
/// let messages = client
///     .channel_messages(channel_id)
///     .before(message_id)
///     .limit(6u16)
///     .await?;
///
/// # Ok(()) }
/// ```
///
/// [`after`]: Self::after
/// [`around`]: Self::around
/// [`before`]: Self::before
/// [`GetChannelMessagesConfigured`]: super::GetChannelMessagesConfigured
/// [`limit`]: Self::limit
#[must_use = "requests must be configured and executed"]
pub struct GetChannelMessages<'a> {
    http: &'a Client,
    path: Result<GetChannelMessagesPath, ValidationError>,
}

impl<'a> GetChannelMessages<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            http,
            path: Ok(GetChannelMessagesPath {
                channel_id,
                limit: None,
            }),
        }
    }

    pub fn after(self, message_id: Id<MessageMarker>) -> GetChannelMessagesConfigured<'a> {
        GetChannelMessagesConfigured::new(self.http, self.path, Some(message_id), None, None)
    }

    pub fn around(self, message_id: Id<MessageMarker>) -> GetChannelMessagesConfigured<'a> {
        GetChannelMessagesConfigured::new(self.http, self.path, None, Some(message_id), None)
    }

    pub fn before(self, message_id: Id<MessageMarker>) -> GetChannelMessagesConfigured<'a> {
        GetChannelMessagesConfigured::new(self.http, self.path, None, None, Some(message_id))
    }

    /// Set the maximum number of messages to retrieve.
    ///
    /// The minimum is 1 and the maximum is 100.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`GetChannelMessages`] error type if the amount
    /// is less than 1 or greater than 100.
    ///
    /// [`GetChannelMessages`]: twilight_validate::request::ValidationErrorType::GetChannelMessages
    pub fn limit(mut self, limit: u16) -> Self {
        self.path = self.path.and_then(|mut path| {
            validate_get_channel_messages_limit(limit)?;
            path.limit = Some(limit);

            Ok(path)
        });

        self
    }
}

impl IntoFuture for GetChannelMessages<'_> {
    type Output = Result<Response<ListBody<Message>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<Message>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetChannelMessages<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetChannelMessagesPath { channel_id, limit } = self.path.map_err(Error::validation)?;

        let path = Path::builder()
            .resource("channels")
            .id(channel_id)
            .resource("messages")
            .parameter(("limit", limit))
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
