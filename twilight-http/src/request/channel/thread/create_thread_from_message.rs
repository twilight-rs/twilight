use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
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
/// When called on a [`GuildText`] channel, this creates a
/// [`PublicThread`].
///
/// When called on a [`GuildAnnouncement`] channel, this creates a [`AnnouncementThread`].
///
/// This request does not work when called on a [`GuildForum`] channel.
///
/// Automatic archive durations are not locked behind the guild's boost level.
///
/// The thread's ID will be the same as its parent message. This ensures only
/// one thread can be created per message.
///
/// [`AnnouncementThread`]: twilight_model::channel::ChannelType::AnnouncementThread
/// [`GuildAnnouncement`]: twilight_model::channel::ChannelType::GuildAnnouncement
/// [`GuildForum`]: twilight_model::channel::ChannelType::GuildForum
/// [`GuildPublicThread`]: twilight_model::channel::ChannelType::GuildPublicThread
/// [`GuildText`]: twilight_model::channel::ChannelType::GuildText
/// [`PublicThread`]: twilight_model::channel::ChannelType::PublicThread
#[must_use = "requests must be configured and executed"]
pub struct CreateThreadFromMessage<'a> {
    channel_id: Id<ChannelMarker>,
    fields: Result<CreateThreadFromMessageFields<'a>, ChannelValidationError>,
    http: &'a Client,
    message_id: Id<MessageMarker>,
}

impl<'a> CreateThreadFromMessage<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
        name: &'a str,
    ) -> Self {
        let fields = Ok(CreateThreadFromMessageFields {
            auto_archive_duration: None,
            name,
        })
        .and_then(|fields| {
            validate_name(name)?;

            Ok(fields)
        });

        Self {
            channel_id,
            fields,
            http,
            message_id,
        }
    }

    /// Set the thread's auto archive duration.
    ///
    /// Automatic archive durations are not locked behind the guild's boost
    /// level.
    pub fn auto_archive_duration(mut self, auto_archive_duration: AutoArchiveDuration) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.auto_archive_duration = Some(auto_archive_duration);
        }

        self
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
        let fields = self.fields.map_err(Error::validation)?;

        Request::builder(&Route::CreateThreadFromMessage {
            channel_id: self.channel_id.get(),
            message_id: self.message_id.get(),
        })
        .json(&fields)
        .build()
    }
}
