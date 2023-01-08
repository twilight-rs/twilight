use crate::{
    client::Client,
    error::Error,
    request::{Request, RequestBuilder, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    channel::{thread::AutoArchiveDuration, Channel},
    id::{
        marker::{ChannelMarker, MessageMarker},
        Id,
    },
};
use twilight_validate::channel::{name as validate_name, ChannelValidationError};

#[derive(Serialize)]
struct CreateThreadFromMessageFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_archive_duration: Option<AutoArchiveDuration>,
    name: &'a str,
}

/// Create a new thread from an existing message.
///
/// When called on a [`GUILD_TEXT`] channel, this creates a
/// [`PUBLIC_THREAD`].
///
/// When called on a [`GUILD_ANNOUNCEMENT`] channel, this creates a [`ANNOUNCEMENT_THREAD`].
///
/// This request does not work when called on a [`GUILD_FORUM`] channel.
///
/// Automatic archive durations are not locked behind the guild's boost level.
///
/// The thread's ID will be the same as its parent message. This ensures only
/// one thread can be created per message.
///
/// [`ANNOUNCEMENT_THREAD`]: twilight_model::channel::ChannelType::ANNOUNCEMENT_THREAD
/// [`GUILD_ANNOUNCEMENT`]: twilight_model::channel::ChannelType::GUILD_ANNOUNCEMENT
/// [`GUILD_FORUM`]: twilight_model::channel::ChannelType::GUILD_FORUM
/// [`GUILD_PUBLIC_THREAD`]: twilight_model::channel::ChannelType::GUILD_PUBLIC_THREAD
/// [`GUILD_TEXT`]: twilight_model::channel::ChannelType::GUILD_TEXT
/// [`PUBLIC_THREAD`]: twilight_model::channel::ChannelType::PUBLIC_THREAD
#[must_use = "requests must be configured and executed"]
pub struct CreateThreadFromMessage<'a> {
    channel_id: Id<ChannelMarker>,
    fields: CreateThreadFromMessageFields<'a>,
    http: &'a Client,
    message_id: Id<MessageMarker>,
}

impl<'a> CreateThreadFromMessage<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
        name: &'a str,
    ) -> Result<Self, ChannelValidationError> {
        validate_name(name)?;

        Ok(Self {
            channel_id,
            fields: CreateThreadFromMessageFields {
                auto_archive_duration: None,
                name,
            },
            http,
            message_id,
        })
    }

    /// Set the thread's auto archive duration.
    ///
    /// Automatic archive durations are not locked behind the guild's boost
    /// level.
    pub const fn auto_archive_duration(
        mut self,
        auto_archive_duration: AutoArchiveDuration,
    ) -> Self {
        self.fields.auto_archive_duration = Some(auto_archive_duration);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    #[deprecated(since = "0.14.0", note = "use `.await` or `into_future` instead")]
    pub fn exec(self) -> ResponseFuture<Channel> {
        self.into_future()
    }
}

impl IntoFuture for CreateThreadFromMessage<'_> {
    type Output = Result<Response<Channel>, Error>;

    type IntoFuture = ResponseFuture<Channel>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreateThreadFromMessage<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::CreateThreadFromMessage {
            channel_id: self.channel_id.get(),
            message_id: self.message_id.get(),
        })
        .json(&self.fields)
        .map(RequestBuilder::build)
    }
}
