use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    channel::thread::ThreadsListing,
    id::{marker::ChannelMarker, Id},
};

/// Returns archived public threads in the channel.
///
/// Requires the [`READ_MESSAGE_HISTORY`] permission.
///
/// Threads are ordered by [`archive_timestamp`] in descending order.
///
/// When called in a [`GuildText`] channel, returns [`PublicThread`]s.
///
/// When called in a [`GuildAnnouncement`] channel, returns [`AnnouncementThread`]s.
///
/// [`AnnouncementThread`]: twilight_model::channel::ChannelType::AnnouncementThread
/// [`archive_timestamp`]: twilight_model::channel::thread::ThreadMetadata::archive_timestamp
/// [`GuildAnnouncement`]: twilight_model::channel::ChannelType::GuildAnnouncement
/// [`GuildText`]: twilight_model::channel::ChannelType::GuildText
/// [`PublicThread`]: twilight_model::channel::ChannelType::PublicThread
/// [`READ_MESSAGE_HISTORY`]: twilight_model::guild::Permissions::READ_MESSAGE_HISTORY
#[must_use = "requests must be configured and executed"]
pub struct GetPublicArchivedThreads<'a> {
    before: Option<&'a str>,
    channel_id: Id<ChannelMarker>,
    http: &'a Client,
    limit: Option<u64>,
}

impl<'a> GetPublicArchivedThreads<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            before: None,
            channel_id,
            http,
            limit: None,
        }
    }

    /// Return threads before this ISO 8601 timestamp.
    pub const fn before(mut self, before: &'a str) -> Self {
        self.before = Some(before);

        self
    }

    /// Maximum number of threads to return.
    pub const fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);

        self
    }
}

impl IntoFuture for GetPublicArchivedThreads<'_> {
    type Output = Result<Response<ThreadsListing>, Error>;

    type IntoFuture = ResponseFuture<ThreadsListing>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetPublicArchivedThreads<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetPublicArchivedThreads {
            before: self.before,
            channel_id: self.channel_id.get(),
            limit: self.limit,
        }))
    }
}
