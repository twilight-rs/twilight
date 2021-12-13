use crate::{
    client::Client, error::Error as HttpError, request::Request, response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    channel::{thread::AutoArchiveDuration, Channel},
    id::{ChannelId, MessageId},
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
/// [`GuildPublicThread`].
///
/// When called on a [`GuildNews`] channel, this creates a [`GuildNewsThread`].
///
/// To use auto archive durations of [`ThreeDays`] or [`Week`], the guild must
/// be boosted.
///
/// The thread's ID will be the same as its parent message. This ensures only
/// one thread can be created per message.
///
/// [`GuildNewsThread`]: twilight_model::channel::ChannelType::GuildNewsThread
/// [`GuildNews`]: twilight_model::channel::ChannelType::GuildNews
/// [`GuildPublicThread`]: twilight_model::channel::ChannelType::GuildPublicThread
/// [`GuildText`]: twilight_model::channel::ChannelType::GuildText
/// [`ThreeDays`]: twilight_model::channel::thread::AutoArchiveDuration::ThreeDays
/// [`Week`]: twilight_model::channel::thread::AutoArchiveDuration::Week
#[must_use = "requests must be configured and executed"]
pub struct CreateThreadFromMessage<'a> {
    channel_id: ChannelId,
    fields: CreateThreadFromMessageFields<'a>,
    http: &'a Client,
    message_id: MessageId,
}

impl<'a> CreateThreadFromMessage<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: ChannelId,
        message_id: MessageId,
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
    /// Values of [`ThreeDays`] and [`Week`] require the guild to be boosted.
    /// The guild's features will indicate if a guild is able to use these
    /// settings.
    ///
    /// [`ThreeDays`]: twilight_model::channel::thread::AutoArchiveDuration::ThreeDays
    /// [`Week`]: twilight_model::channel::thread::AutoArchiveDuration::Week
    pub const fn auto_archive_duration(
        mut self,
        auto_archive_duration: AutoArchiveDuration,
    ) -> Self {
        self.fields.auto_archive_duration = Some(auto_archive_duration);

        self
    }

    fn request(&self) -> Result<Request, HttpError> {
        let request = Request::builder(&Route::CreateThreadFromMessage {
            channel_id: self.channel_id.get(),
            message_id: self.message_id.get(),
        })
        .json(&self.fields)?;

        Ok(request.build())
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Channel> {
        match self.request() {
            Ok(request) => self.http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}
