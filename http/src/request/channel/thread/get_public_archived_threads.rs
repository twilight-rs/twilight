use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use twilight_model::{channel::thread::ThreadsListing, id::ChannelId};

/// Returns archived public threads in the channel.
///
/// Requires the [`READ_MESSAGE_HISTORY`] permission.
///
/// Threads are ordered by [`archive_timestamp`] in descending order.
///
/// When called in a [`GuildText`] channel, returns [`GuildPublicThread`]s.
///
/// When called in a [`GuildNews`] channel, returns [`GuildNewsThread`]s.
///
/// [`archive_timestamp`]: twilight_model::channel::thread::ThreadMetadata::archive_timestamp
/// [`GuildNews`]: twilight_model::channel::ChannelType::GuildNews
/// [`GuildNewsThread`]: twilight_model::channel::ChannelType::GuildNewsThread
/// [`GuildPublicThread`]: twilight_model::channel::ChannelType::GuildPublicThread
/// [`GuildText`]: twilight_model::channel::ChannelType::GuildText
/// [`READ_MESSAGE_HISTORY`]: twilight_model::guild::Permissions::READ_MESSAGE_HISTORY
#[must_use = "requests must be configured and executed"]
pub struct GetPublicArchivedThreads<'a> {
    before: Option<&'a str>,
    channel_id: ChannelId,
    http: &'a Client,
    limit: Option<u64>,
}

impl<'a> GetPublicArchivedThreads<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: ChannelId) -> Self {
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

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ThreadsListing> {
        let request = Request::from_route(&Route::GetPublicArchivedThreads {
            before: self.before,
            channel_id: self.channel_id.get(),
            limit: self.limit,
        });

        self.http.request(request)
    }
}
