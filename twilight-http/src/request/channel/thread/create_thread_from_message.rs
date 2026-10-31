use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    channel::{Channel, thread::AutoArchiveDuration},
    id::{
        Id,
        marker::{ChannelMarker, MessageMarker},
    },
};
use twilight_validate::{
    channel::{ChannelValidationError, name as validate_name},
    request::{ValidationError, audit_reason as validate_audit_reason},
};

#[derive(Serialize)]
struct CreateThreadFromMessageBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_archive_duration: Option<AutoArchiveDuration>,
    name: &'a str,
}

struct CreateThreadFromMessagePath {
    channel_id: Id<ChannelMarker>,
    message_id: Id<MessageMarker>,
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
/// [`GuildText`]: twilight_model::channel::ChannelType::GuildText
/// [`PublicThread`]: twilight_model::channel::ChannelType::PublicThread
#[must_use = "requests must be configured and executed"]
pub struct CreateThreadFromMessage<'a> {
    body: Result<CreateThreadFromMessageBody<'a>, ChannelValidationError>,
    http: &'a Client,
    path: CreateThreadFromMessagePath,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> CreateThreadFromMessage<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
        name: &'a str,
    ) -> Self {
        let body = Ok(CreateThreadFromMessageBody {
            auto_archive_duration: None,
            name,
        })
        .and_then(|body| {
            validate_name(name)?;

            Ok(body)
        });

        Self {
            body,
            http,
            path: CreateThreadFromMessagePath {
                channel_id,
                message_id,
            },
            reason: Ok(None),
        }
    }

    /// Set the thread's auto archive duration.
    ///
    /// Automatic archive durations are not locked behind the guild's boost
    /// level.
    pub const fn auto_archive_duration(
        mut self,
        auto_archive_duration: AutoArchiveDuration,
    ) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.auto_archive_duration = Some(auto_archive_duration);
        }

        self
    }
}

impl<'a> AuditLogReason<'a> for CreateThreadFromMessage<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

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
        let body = self.body.map_err(Error::validation)?;

        let mut request = Request::builder_from_route(&Route::CreateThreadFromMessage {
            channel_id: self.path.channel_id.get(),
            message_id: self.path.message_id.get(),
        })
        .json(&body);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
