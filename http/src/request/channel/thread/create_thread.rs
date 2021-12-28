use crate::{
    client::Client,
    error::Error,
    request::{Request, RequestBuilder, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    channel::{thread::AutoArchiveDuration, Channel, ChannelType},
    id::{marker::ChannelMarker, Id},
};
use twilight_validate::channel::{
    is_thread as validate_is_thread, name as validate_name, ChannelValidationError,
};

#[derive(Serialize)]
struct CreateThreadFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_archive_duration: Option<AutoArchiveDuration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invitable: Option<bool>,
    #[serde(rename = "type")]
    kind: ChannelType,
    name: &'a str,
}

/// Start a thread that is not connected to a message.
///
/// Values of [`ThreeDays`] and [`Week`] require the guild to be boosted.  The
/// guild's features will indicate if a guild is able to use these settings.
///
/// To make a [`GuildPrivateThread`], the guild must also have the
/// `PRIVATE_THREADS` feature.
///
/// [`GuildPrivateThread`]: twilight_model::channel::ChannelType::GuildPrivateThread
/// [`ThreeDays`]: twilight_model::channel::thread::AutoArchiveDuration::ThreeDays
/// [`Week`]: twilight_model::channel::thread::AutoArchiveDuration::Week
#[must_use = "requests must be configured and executed"]
pub struct CreateThread<'a> {
    channel_id: Id<ChannelMarker>,
    fields: CreateThreadFields<'a>,
    http: &'a Client,
}

impl<'a> CreateThread<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        name: &'a str,
        kind: ChannelType,
    ) -> Result<Self, ChannelValidationError> {
        validate_name(name)?;

        validate_is_thread(kind)?;

        Ok(Self {
            channel_id,
            fields: CreateThreadFields {
                auto_archive_duration: None,
                invitable: None,
                kind,
                name,
            },
            http,
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

    /// Whether non-moderators can add other non-moderators to a thread.
    pub const fn invitable(mut self, invitable: bool) -> Self {
        self.fields.invitable = Some(invitable);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Channel> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreateThread<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::CreateThread {
            channel_id: self.channel_id.get(),
        })
        .json(&self.fields)
        .map(RequestBuilder::build)
    }
}
